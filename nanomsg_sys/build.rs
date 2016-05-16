fn main() {
    println!("cargo:rustc-flags=-L ../nanomsg-0.9-beta/build -l nanomsg");
}