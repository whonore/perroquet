#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(nonstandard_style)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused)]
#![warn(clippy::all, clippy::pedantic)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::use_self)]
#![warn(clippy::if_then_some_else_none)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::if_not_else)]
#![allow(clippy::pub_enum_variant_names)]
#![allow(clippy::similar_names)]
#![allow(clippy::too_many_lines)]

mod cli;

fn main() {
    let opts = cli::parse_args();
    println!("{:?}", opts);
}
