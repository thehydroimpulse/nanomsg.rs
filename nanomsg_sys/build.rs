extern crate cmake;
extern crate gcc;
extern crate pkg_config;

use std::env;
use std::path::Path;
use std::process::Command;


#[cfg(feature = "bundled")]
fn main() {
    let target = env::var("TARGET").unwrap();

    // TODO: Determine whether we'd rather always do a fresh clone.
    // TODO: Determine whether we wouldn't rather use a submodule.
    if !Path::new("nanomsg/.git").exists() {
        // Panic if we can't clone nanomsg
        let _ = Command::new("git")
            .args(&["clone", "-b", "1.0.0", "--depth", "1", "https://github.com/nanomsg/nanomsg.git"])
            .status().unwrap();
    }

    let dst = cmake::Config::new("nanomsg")
        .define("NN_STATIC_LIB", "ON")
        .define("NN_ENABLE_DOC", "OFF")
        .define("NN_TESTS", "OFF")
        .build();

    if target.contains("windows") {
        println!("cargo:rustc-link-lib=mswsock");
    }

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
}

#[cfg(not(feature = "bundled"))]
fn main() {
    // Attempt to use pkg_config to locate nanomsg (search location can be set via environment)
    if pkg_config::find_library("nanomsg").is_err() {
        // If that failed we have some reasonable looking defaults.
        let target = env::var("TARGET").unwrap();
        let windows = target.contains("windows");
        if windows {
            println!("cargo:rustc-link-lib=nanomsg");
            println!("cargo:rustc-link-search=C:/Program Files (x86)/nanomsg/lib");
        } else {
            println!("cargo:rustc-flags=-L /usr/local/lib -l nanomsg");
        }
    }
    //println!("cargo:rustc-flags=-L /usr/local/lib -l nanomsg");
}
