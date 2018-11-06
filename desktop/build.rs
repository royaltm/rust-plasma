use std::env;
use std::path::PathBuf;

const SDL2_WINDOWS_DIR: &'static str = "sdl-2.0.8-windows";

fn main() {
    if cfg!(target_os = "windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();

        lib_dir.push(SDL2_WINDOWS_DIR);

        if cfg!(target_arch = "x86_64") {
            lib_dir.push("x64");
        }
        else if cfg!(target_arch = "x86") {
            lib_dir.push("x86");
        }
        else {
            return;
        }

        let dll_dir = lib_dir.clone();

        if cfg!(target_env = "msvc") {
            lib_dir.push("msvc");
        }
        else if cfg!(target_env = "gnu") {
            lib_dir.push("gnu-mingw");
        }
        else {
            return;
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir")  {
            let entry_path = entry.expect("Invalid fs entry").path();
            if let Some(file_name) = entry_path.file_name() {
                let mut new_file_path = manifest_dir.clone();
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
                }
            }
        }
    }
}