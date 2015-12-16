fn main() {
    println!("cargo:rustc-flags=-L ../nanomsg/.libs -l nanomsg");
}
