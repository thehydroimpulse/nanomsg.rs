fn main() {
    println!("cargo:rustc-flags=-L ../nanomsg-0.6-beta/.libs -l nanomsg");
}