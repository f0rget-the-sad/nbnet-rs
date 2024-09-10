use std::env;
use std::path::PathBuf;

fn main() {
    println!("HELLO from build.rs");
    println!("Out dir is: '{:?}'", env::var_os("OUT_DIR").unwrap());
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    /*
    cc::Build::new()
        .file("nbnet/nbnet.h")
        .define("NBNET_IMPL", None)
        .out_dir(dst.join("build"))
        .warnings(false)
        // XXX: make drivers features
        // UDP driver
        .flag("-includenbnet/net_drivers/udp.h")
        // XXX: allow user to define logger?
        .flag("-includestdio.h")
        .define("NBN_LogInfo", "printf")
        .define("NBN_LogError", "printf")
        .define("NBN_LogWarning", "printf")
        .define("NBN_LogDebug", "printf")
        .define("NBN_LogTrace", "printf")
        .compile("nbnet");
    */
    cc::Build::new()
        .include("nbnet")
        .file("nbnet.c")
        .out_dir(dst.join("build"))
        .warnings(false)
        .compile("nbnet");
}
