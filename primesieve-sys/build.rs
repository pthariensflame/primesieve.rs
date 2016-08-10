extern crate gcc;

fn main() { gcc::compile_library("libprimesieve_auxbind.a", &["primesieve_auxbind.c"]); }
