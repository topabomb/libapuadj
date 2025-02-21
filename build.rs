use std::path::PathBuf;
use std::{env, fs};

fn main() {
    const LIB_NAME: &str = "libryzenadj";
    let dst = cmake::Config::new("RyzenAdj")
        .define("BUILD_SHARED_LIBS", "ON")
        .profile("Release")
        .build_target(LIB_NAME)
        .build();
    //panic!("dst: {:?}", dst.display());

    // Tell cargo to look for shared libraries in the specified directory
    //println!("cargo:rustc-link-search={:}/build/Release",dst.display());
    println!("cargo:rustc-link-search={:}/build/Release", dst.display());

    // Tell cargo to tell rustc to link the libryzenadj
    // shared library.
    println!("cargo:rustc-link-lib=libryzenadj");
    // static library
    //println!("cargo:rustc-link-lib=static=libryzenadj");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    //copy dll to target dir
    let source_dll_path = format!("{}/build/Release/{}.dll", dst.display(), LIB_NAME);
    if !fs::exists(&source_dll_path).expect("fs::exists err.") {
        panic!("{} not found", source_dll_path)
    };
    let target_dir = out_path.parent().unwrap().parent().unwrap().parent().unwrap();
    fs::copy(&source_dll_path, target_dir.join(format!("{}.dll", LIB_NAME))).expect("fs::copy dll file err.");
    fs::copy("RyzenAdj/win32/inpoutx64.dll",target_dir.join("inpoutx64.dll")).expect("fs::copy inpoutx64.dll file err.");
    fs::copy("RyzenAdj/win32/WinRing0x64.dll",target_dir.join("WinRing0x64.dll")).expect("fs::copy WinRing0x64.dll file err.");
    fs::copy("RyzenAdj/win32/WinRing0x64.sys",target_dir.join("WinRing0x64.sys")).expect("fs::copy WinRing0x64.sys file err.");

    
}
