use std::{fs, path::PathBuf};

use bindgen::CargoCallbacks;

fn main() {
    // Create meson build files
    if !std::process::Command::new("meson")
        .arg("setup")
        .arg("c_build")
        .arg("SolisPhysics")
        .arg("--wipe")
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
        .arg("c_build")
        .output()
        .expect("could not spawn `meson`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not generate meson build files");
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

    let _ = fs::create_dir_all("bindgen");
    let out_path = PathBuf::from("bindgen/bindgen_ffi_bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Could not write bindings to file.");
}
