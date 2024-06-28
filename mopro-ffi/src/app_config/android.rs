use std::fs;
use std::path::Path;
use std::process::Command;

use crate::app_config::install_ndk;

use super::{install_arch, mktemp_local};

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
        let out_lib_path = Path::new(&build_dir).join(Path::new(&format!(
            "{}/{}/{}/libmopro_bindings.so",
            build_dir, arch, mode
        )));

        if !bindings_dest.exists() {
            fs::create_dir_all(&bindings_out).expect("Failed to create directory");
        }
        if !bindings_dest.join(Path::new("jniLibs")).exists() {
            fs::create_dir_all(bindings_out.join(Path::new("jniLibs")))
                .expect("Failed to create directory");
        }
        if !bindings_dest
            .join(Path::new("jniLibs"))
            .join(Path::new(&folder))
            .exists()
        {
            fs::create_dir_all(
                bindings_dest
                    .join(Path::new("jniLibs"))
                    .join(Path::new(&folder)),
            )
            .expect("Failed to create directory");
        }
        let out_lib_dest = bindings_dest.join(Path::new(&format!(
            "jniLibs/{}/libmopro_bindings.so",
            folder
        )));
        fs::copy(&out_lib_path, &out_lib_dest).expect("Failed to copy file");

        if !bindings_dest
            .join(Path::new("uniffi"))
            .join(Path::new("mopro"))
            .exists()
        {
            fs::create_dir_all(
                &bindings_dest
                    .join(Path::new("uniffi"))
                    .join(Path::new("mopro")),
            )
            .expect("Failed to create directory");
        }
        fs::write(
            bindings_dest
                .join(Path::new("uniffi"))
                .join(Path::new("mopro"))
                .join("mopro.kt"),
            MOPRO_KOTLIN,
        )
        .expect("Failed to write mopro.swift");
    }
}
