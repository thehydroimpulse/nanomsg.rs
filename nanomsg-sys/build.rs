fn main() {
    println!("cargo:rustc-flags=-L ../nanomsg-0.5-beta/.libs -l nanomsg");
}