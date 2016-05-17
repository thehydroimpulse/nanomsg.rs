fn main() {
    // let target = env::var("TARGET").unwrap();
    // let windows = target.contains("windows");
    //
    // if windows {
    //    ???
    // } else {
    //    println!("cargo:rustc-flags=-L /usr/local/lib -l nanomsg");
    // }
    // TODO : see https://github.com/thehydroimpulse/nanomsg.rs/issues/143
    // Using pkg-config seems like a good idea
    println!("cargo:rustc-flags=-L /usr/local/lib -l nanomsg");
}