use std::fs;
use std::path::Path;
use std::process::Command;

use super::{cleanup_tmp_local, install_arch, install_ndk, mktemp_local};

pub const MOPRO_KOTLIN: &str = include_str!("../../KotlinBindings/uniffi/mopro/mopro.kt");

pub fn build() {
    let cwd = std::env::current_dir().unwrap();
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or(cwd.to_str().unwrap().to_string());
    let build_dir = format!("{}/build", manifest_dir);
    let build_dir_path = Path::new(&build_dir);
    let work_dir = mktemp_local(&build_dir_path);
    // let kotlin_bindings_dir = work_dir.join(Path::new("KotlinBindings"));
    let bindings_out = work_dir.join("MoproAndroidBindings");
    let bindings_dest = Path::new(&manifest_dir).join("MoproAndroidBindings");

    let target_archs = vec![
        "x86_64-linux-android",
        "i686-linux-android",
        "armv7-linux-androideabi",
        "aarch64-linux-android",
    ];

    let mode;
    if let Ok(configuration) = std::env::var("CONFIGURATION") {
        mode = match configuration.as_str() {
            "Debug" => "debug",
            "Release" => "release",
            "debug" => "debug",
            "release" => "release",
            _ => panic!("unknown configuration"),
        };
    } else {
        mode = "debug";
    }

    install_ndk();
    for arch in target_archs {
        install_arch(arch.to_string());
        let mut build_cmd = Command::new("cargo");
        build_cmd.arg("ndk").arg("-t").arg(arch);
        if mode == "release" {
            build_cmd.arg("--release");
        }
        build_cmd
            .arg("build")
            .arg("--lib")
            .env("CARGO_BUILD_TARGET_DIR", &build_dir)
            .env("CARGO_BUILD_TARGET", arch)
            .spawn()
            .expect("Failed to spawn cargo build")
            .wait()
            .expect("cargo build errored");

        let folder: String;
        match arch {
            "x86_64-linux-android" => {
                folder = String::from("x86_64");
            }
            "i686-linux-android" => {
                folder = String::from("x86");
            }
            "armv7-linux-androideabi" => {
                folder = String::from("armeabi-v7a");
            }
            "aarch64-linux-android" => {
                folder = String::from("arm64-v8a");
            }
            _ => panic!("Unknown target architecture"),
        }
        let out_lib_path = Path::new(&build_dir).join(format!(
            "{}/{}/{}/libmopro_bindings.so",
            build_dir, arch, mode
        ));

        let out_lib_dest = bindings_out.join(format!("jniLibs/{}/libmopro_bindings.so", folder));

        // Create necessary directories and copy the library
        fs::create_dir_all(out_lib_dest.parent().unwrap())
            .expect("Failed to create jniLibs directory");
        fs::copy(&out_lib_path, &out_lib_dest).expect("Failed to copy file");
    }

    fs::create_dir_all(&bindings_out.join("uniffi").join("mopro"))
        .expect("Failed to create directory");

    fs::write(
        bindings_out.join("uniffi").join("mopro").join("mopro.kt"),
        MOPRO_KOTLIN,
    )
    .expect("Failed to write mopro.kt");

    if let Ok(info) = fs::metadata(&bindings_dest) {
        if !info.is_dir() {
            panic!("bindings directory exists and is not a directory");
        }
        fs::remove_dir_all(&bindings_dest).expect("Failed to remove bindings directory");
    }
    fs::rename(&bindings_out, &bindings_dest).expect("Failed to move bindings into place");
    cleanup_tmp_local(&build_dir_path)
}
