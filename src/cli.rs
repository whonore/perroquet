use std::collections::HashSet;
use std::path::{Path, PathBuf};

use clap::{
    crate_authors, crate_name, crate_version, value_t, App, AppSettings, Arg, Error, ErrorKind,
};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Input {
    File(PathBuf),
    Stdin,
}

#[derive(Debug)]
pub struct Opts {
    inputs: HashSet<Input>,
    from: Option<usize>,
    upto: Option<usize>,
    check: bool,
}

impl Opts {
    fn new() -> Self {
        Self {
            inputs: HashSet::new(),
            from: None,
            upto: None,
            check: false,
        }
    }

    const fn from(mut self, line: usize) -> Self {
        self.from = Some(line);
        self
    }

    const fn upto(mut self, line: usize) -> Self {
        self.upto = Some(line);
        self
    }

    const fn check(mut self) -> Self {
        self.check = true;
        self
    }

    fn add_file<P>(mut self, file: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.inputs.insert(Input::File(file.into()));
        self
    }

    fn add_stdin(mut self) -> Self {
        self.inputs.insert(Input::Stdin);
        self
    }
}

struct ExcludeFiles<'a> {
    candidates: HashSet<PathBuf>,
    excludes: &'a HashSet<PathBuf>,
}

#[derive(Debug)]
enum ExcludeRes {
    // Found a non-excluded file
    File(PathBuf),
    // Found a non-excluded directory and added its children to the pool of
    // potential paths
    Dir(PathBuf),
}

impl<'a> ExcludeFiles<'a> {
    fn new<I, S>(candidates: I, excludes: &'a HashSet<PathBuf>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let candidates = candidates
            .into_iter()
            .filter_map(|input| {
                let path = Path::new(input.as_ref()).canonicalize();
                if path.is_err() {
                    eprintln!(
                        "{}",
                        Error::with_description(
                            &format!("{} -- {}", input.as_ref(), path.as_ref().unwrap_err()),
                            ErrorKind::InvalidValue,
                        )
                    );
                }
                path.ok()
            })
            .collect::<HashSet<_>>();

        Self {
            candidates,
            excludes,
        }
    }
}

impl Iterator for ExcludeFiles<'_> {
    type Item = ExcludeRes;

    fn next(&mut self) -> Option<Self::Item> {
        // Pop entries from `self.candidates` until a non-excluded file or
        // directory is found.
        let mut drain = self.candidates.drain();
        let res = loop {
            if let Some(path) = drain.next() {
                if self.excludes.contains(&path) {
                    continue;
                }

                if path.is_file() {
                    break Some(ExcludeRes::File(path));
                } else if path.is_dir() {
                    break Some(ExcludeRes::Dir(path));
                }
            } else {
                break None;
            }
        };

        // Rebuild the set of candidates.
        self.candidates = drain.collect();

        // Add the directory's entries to `self.candidates`. Only take files
        // with a `.v` extension.
        if let Some(ExcludeRes::Dir(ref dir)) = res {
            if let Ok(entries) = dir.read_dir() {
                for path in entries
                    .flatten() // Drop Err results
                    .map(|entry| entry.path())
                    .filter(|path| !path.is_file() || path.extension().unwrap_or_default() == "v")
                {
                    self.candidates.insert(path);
                }
            }
        }

        res
    }
}

fn validate_line<S: AsRef<str>>(v: S) -> Result<(), String> {
    if v.as_ref().parse::<usize>().is_err() {
        Err("not a valid line number".into())
    } else {
        Ok(())
    }
}

pub fn parse_args() -> Opts {
    let mut opts = Opts::new();
    let args = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about("Indents Coq code")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("input")
                .multiple(true)
                .help("The files or directories to indent"),
        )
        .arg(
            Arg::with_name("exclude")
                .long("exclude")
                .takes_value(true)
                .number_of_values(1)
                .multiple(true)
                .help("Exclude a file or directory"),
        )
        .arg(
            Arg::with_name("check")
                .long("check")
                .help("Check that files are indented correctly"),
        )
        .arg(
            Arg::with_name("from")
                .long("from")
                .takes_value(true)
                .value_name("LINE")
                .validator(validate_line)
                .help("Line to begin indenting from"),
        )
        .arg(
            Arg::with_name("upto")
                .long("upto")
                .takes_value(true)
                .value_name("LINE")
                .validator(validate_line)
                .help("Line to stop indenting at"),
        )
        .get_matches();

    if let Ok(line) = value_t!(args.value_of("from"), usize) {
        opts = opts.from(line);
    }
    if let Ok(line) = value_t!(args.value_of("upto"), usize) {
        opts = opts.upto(line);
    }
    if args.is_present("check") {
        opts = opts.check();
    }

    let excludes = args.values_of("exclude").map_or(HashSet::new(), |vs| {
        vs.filter_map(|path| Path::new(path).canonicalize().ok())
            .collect()
    });

    #[allow(clippy::redundant_closure_for_method_calls)]
    let inputs = args
        .values_of("input")
        .map_or(HashSet::new(), |vs| vs.collect());
    if inputs.contains("-") {
        opts = opts.add_stdin();
    }
    let inputs = inputs.iter().filter(|v| **v != "-");

    for input in ExcludeFiles::new(inputs, &excludes) {
        if let ExcludeRes::File(path) = input {
            opts = opts.add_file(path);
        }
    }

    if (opts.from.is_some() || opts.upto.is_some()) && opts.inputs.len() > 1 {
        Error::with_description(
            "--from and --upto cannot be used with more than one input file.",
            ErrorKind::ArgumentConflict,
        )
        .exit()
    }

    opts
}
