extern crate gcc;

fn main() {
    gcc::compile_library("libprimesieve_rust_bindings.a", &["libprimesieve_rust_bindings.c"]);
}
