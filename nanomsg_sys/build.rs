fn main() {
    println!("cargo:rustc-flags=-L ../nanomsg-0.7-beta/.libs -l nanomsg");
}