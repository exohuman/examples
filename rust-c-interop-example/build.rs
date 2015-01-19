// build.rs

fn main() {
    println!("cargo:rustc-flags= -L lib/exampleC/lib");
}
