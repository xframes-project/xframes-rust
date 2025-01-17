fn main() {
    println!("cargo:rustc-link-lib=dylib=xframesshared");
    println!("cargo:rustc-link-search=native=./");
}