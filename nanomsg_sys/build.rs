#[cfg(not(windows))]
fn main() {
    println!("cargo:rustc-flags=-L /usr/local/lib -l nanomsg");
}

#[cfg(windows)]
fn main() {
    println!("cargo:rustc-link-lib=nanomsg");
    println!("cargo:rustc-link-search=C:/Program Files (x86)/nanomsg/lib");
}

    // let target = env::var("TARGET").unwrap();
    // let windows = target.contains("windows");
    //
    // if windows {
    //    ???
    // } else {
    //    println!("cargo:rustc-flags=-L /usr/local/lib -l nanomsg");
    // }
    // TODO : see https://github.com/thehydroimpulse/nanomsg.rs/issues/143
    // https://github.com/alexcrichton/git2-rs/blob/master/libgit2-sys/build.rs#L29
    // Using pkg-config seems like a good idea
