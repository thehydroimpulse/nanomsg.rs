fn main() {
    println!("cargo:rustc-flags=-L ../nanomsg-0.8-beta/.libs -l nanomsg");
}