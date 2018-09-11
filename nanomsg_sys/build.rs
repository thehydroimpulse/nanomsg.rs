extern crate cmake;
extern crate gcc;
extern crate pkg_config;

use std::env;
#[cfg(feature = "bundled")]
use std::path::Path;

#[cfg(feature = "bundled")]
const NANOMSG_VERSION: &'static str = "1.1.4";

#[cfg(feature = "bundled")]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();

    let clone_path = Path::new(&out_dir).join("nanomsg_upstream");

    // TODO: Determine whether we'd rather always do a fresh clone.
    // TODO: Determine whether we wouldn't rather use a submodule.
    if !clone_path.join(".git").exists() {
        // Panic if we can't clone nanomsg
        let status = ::std::process::Command::new("git")
            .args(
                &[
                    "clone",
                    "-b",
                    NANOMSG_VERSION,
                    "--depth",
                    "1",
                    "https://github.com/nanomsg/nanomsg.git",
                    clone_path.to_str().unwrap(),
                ],
            )
            .status()
            .unwrap();

        if !status.success() {
            panic!("git clone of nanomsg was not successful");
        }
    } else {
        let status = ::std::process::Command::new("git")
            .current_dir(clone_path.clone())
            .args(&["checkout", NANOMSG_VERSION])
            .status()
            .unwrap();

        if !status.success() {
            panic!(
                "git checkout of nanomsg {} was not successful",
                NANOMSG_VERSION
            );
        }
    }

    let getaddrinfo_a_flag = if cfg!(feature = "no_anl") {
        "OFF"
    } else {
        "ON"
    };

    let mut config = cmake::Config::new(clone_path);
    config
        .define("NN_STATIC_LIB", "ON")
        .define("NN_ENABLE_DOC", "OFF")
        .define("NN_ENABLE_GETADDRINFO_A", getaddrinfo_a_flag)
        .define("NN_TESTS", "OFF");

    if env::var("CARGO_CFG_TARGET_ENV").unwrap() == "musl" {
        config.define("CMAKE_SKIP_INSTALL_RPATH", "ON");
    }

    let dst = config.build();

    if target.contains("windows") {
        println!("cargo:rustc-link-lib=mswsock");
    }

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
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
}
