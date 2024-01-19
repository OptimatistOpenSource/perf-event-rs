extern crate bindgen;

use std::path::Path;
use std::{env, fs};

fn bindgen(linux_headers_path: &str) {
    let bindings_output_path = "src/syscall/bindings/bindgen.rs";
    // Try delete `bindgen.rs` for every build
    let _ = fs::remove_file(bindings_output_path);

    let builder = bindgen::Builder::default()
        .derive_default(true)
        .generate_comments(false)
        .prepend_enum_name(false)
        .header("wrapper.h");

    if linux_headers_path != "/usr/include" {
        builder.clang_arg(format!("-I{}", linux_headers_path))
    } else {
        builder
    }
    .generate()
    .unwrap_or_else(|e| panic!("Failed to generate bindings: {}", e))
    .write_to_file(bindings_output_path)
    .unwrap_or_else(|e| panic!("Failed to write {}: {}", bindings_output_path, e));
}

fn main() {
    // Check target OS
    match env::var("CARGO_CFG_TARGET_OS") {
        Ok(target_os) => match target_os.as_str() {
            "linux" | "android" => {}
            target_os => panic!("Invalid target OS: {:?}", target_os),
        },
        Err(e) => {
            panic!("Unknown target OS: {}", e);
        }
    };

    let mut selected_linux_feature: Option<&str> = None;

    macro_rules! check_selected_linux_feature {
        ($linux:expr) => {
            if cfg!(feature = $linux) && selected_linux_feature.is_none() {
                selected_linux_feature = Some($linux);
            }
        };
    }

    check_selected_linux_feature!("linux-6.3");
    check_selected_linux_feature!("linux-6.0");
    check_selected_linux_feature!("linux-5.16");
    check_selected_linux_feature!("linux-5.13");
    check_selected_linux_feature!("linux-5.12");
    check_selected_linux_feature!("linux-5.11");
    check_selected_linux_feature!("linux-5.9");
    check_selected_linux_feature!("linux-5.8");
    check_selected_linux_feature!("linux-5.7");
    check_selected_linux_feature!("linux-5.5");
    check_selected_linux_feature!("linux-5.4");

    let linux_headers_path = if env::var("LINUX_HEADERS_PATH").is_ok() {
        let path = format!("{}/include", env::var("LINUX_HEADERS_PATH").unwrap());
        let path = Path::new(&path).canonicalize().unwrap();
        path.to_str().unwrap().to_string()
    } else {
        // TODO: get the right location of libc in the building system.
        // as different linux distros have different locations of libc header files.
        // on Ubuntu or Fedora, the default location is `/usr/include`
        // while on other distros like nix, they may have different locations.
        "/usr/include".to_string()
    };

    let linux_version_header_file_path = format!("{}/{}", linux_headers_path, "linux/version.h");
    let contents = fs::read_to_string(linux_version_header_file_path).unwrap();

    let (major, patch_level, sub_level) = {
        let mut major = None;
        let mut patch_level = None;
        let mut sub_level = None;

        for line in contents.lines() {
            let values: Vec<&str> = line.trim().split_ascii_whitespace().collect();
            match values[1] {
                "LINUX_VERSION_MAJOR" => major = Some(values[2]),
                "LINUX_VERSION_PATCHLEVEL" => patch_level = Some(values[2]),
                "LINUX_VERSION_SUBLEVEL" => sub_level = Some(values[2]),
                _ => (),
            }
        }

        (
            major.expect("Failed to parse LINUX_VERSION_MAJOR"),
            patch_level.expect("Failed to parse LINUX_VERSION_PATCHLEVEL"),
            sub_level.expect("Failed to parse LINUX_VERSION_SUBLEVEL"),
        )
    };

    if let Some(selected_linux_feature) = selected_linux_feature {
        let selected_linux_version = selected_linux_feature.replace("linux-", "");
        let mut split = selected_linux_version.split('.');
        let selected_linux_major = split.next().unwrap();
        let selected_linux_patch_level = split.next().unwrap();

        if selected_linux_major != major || selected_linux_patch_level != patch_level {
            println!(
                "cargo:warning=Selected Linux feature ({}) may not compatible with compile against Linux version: {}.{}.{}. ",
                selected_linux_feature,
                major,
                patch_level,
                sub_level
            );
            println!(
                "cargo:warning=To set another linux header file location, run `LINUX_HEADERS_PATH=/path/to/directory cargo build --features={}`",
                selected_linux_feature
            )
        }
    } else {
        // select default features based on parsed linux version
        let major_five_patch_levels = [4, 5, 7, 8, 11, 12, 13, 16];
        let major_six_patch_levels = [0, 3];
        match major {
            "5" => {
                major_five_patch_levels.map(|v| {
                    if patch_level.parse::<usize>().unwrap() >= v {
                        println!("cargo:rustc-cfg=feature=\"linux-5.{}\"", v)
                    }
                });
            }
            "6" => {
                major_five_patch_levels
                    .map(|x| println!("cargo:rustc-cfg=feature=\"linux-5.{}\"", x));

                major_six_patch_levels.map(|x| {
                    if patch_level.parse::<usize>().unwrap() >= x {
                        println!("cargo:rustc-cfg=feature=\"linux-6.{}\"", x)
                    }
                });
            }
            major => panic!("Invalid Linux major version: {}", major),
        };
    }

    bindgen(&linux_headers_path)
}
