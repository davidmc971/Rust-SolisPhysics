use std::{env, fs, path::PathBuf};

use bindgen::CargoCallbacks;

fn main() {
    // Prepare folders
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);
    let c_build = PathBuf::from(&out_dir).join("c_build");
    let _ = fs::create_dir_all(&c_build);
    let linker_path = &c_build.canonicalize().expect("Cannot canonicalize path.");
    let target_file = &out_path.join("bindgen_ffi_bindings.rs");

    // Tell rust to link our lib
    println!("cargo:rustc-link-search={}", &linker_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=solis_physics");

    // Create meson build files
    if !std::process::Command::new("meson")
        .arg("setup")
        .arg(&c_build)
        .arg("SolisPhysics")
        .arg("--wipe")
        .arg("--default-library")
        .arg("static")
        .output()
        .expect("could not spawn `meson`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not generate meson build files");
    }

    // Run ninja to compile
    if !std::process::Command::new("ninja")
        .arg("-C")
        .arg(&c_build)
        .output()
        .expect("could not spawn `ninja`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile SolisPhysics");
    }

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("SolisPhysics/Source/Math.h")
        .header("SolisPhysics/Source/CollisionShapes2D.h")
        .allowlist_function("Sol_.*")
        .allowlist_type("Sol_.*")
        .allowlist_var("Sol_.*")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(target_file)
        .expect("Could not write bindings to file.");
}
