extern crate bindgen;

use std::path::Path;
use std::{env, fs};

fn main() {
    match env::var("CARGO_CFG_TARGET_OS").as_ref().map(|x| &**x) {
        Ok("linux") | Ok("android") => {
            let mut user_selected_linux_feature = None;

            macro_rules! check_selected_linux_feature {
                ($linux:expr) => {
                    if cfg!(feature = $linux) && user_selected_linux_feature.is_none() {
                        user_selected_linux_feature = Some($linux);
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

            let mut major = None;
            let mut patch_level = None;
            let mut sublevel = None;

            let linux_headers_path = {
                if env::var("LINUX_HEADERS_PATH").is_ok() {
                    let path = format!("{}/include", env::var("LINUX_HEADERS_PATH").unwrap());
                    let path = Path::new(&path).canonicalize().unwrap();
                    path.to_str().unwrap().to_string()
                } else {
                    // TODO: get the right location of libc in the building system.
                    // as different linux distros have different locations of libc header files.
                    // on Ubuntu or Fedora, the default location is `/usr/include`
                    // while on other distros like nix, they may have different locations.
                    "/usr/include".to_string()
                }
            };
            let linux_version_header_file_path =
                format!("{}/{}", linux_headers_path, "linux/version.h");
            let contents = fs::read_to_string(linux_version_header_file_path).unwrap();
            for line in contents.lines() {
                let values: Vec<&str> = line.trim().split_ascii_whitespace().collect();
                match values[1] {
                    "LINUX_VERSION_MAJOR" => major = Some(values[2]),
                    "LINUX_VERSION_PATCHLEVEL" => patch_level = Some(values[2]),
                    "LINUX_VERSION_SUBLEVEL" => sublevel = Some(values[2]),
                    _ => (),
                }
            }

            if let Some(selected_linux_feature) = user_selected_linux_feature {
                let selected_linux_version: Vec<&str> =
                    selected_linux_feature.split('-').collect::<Vec<&str>>()[1]
                        .split('.')
                        .collect();
                if selected_linux_version[0] != major.unwrap()
                    || selected_linux_version[1] != patch_level.unwrap()
                {
                    println!(
                        "cargo:warning=Selected Linux feature ({}) may not compatible with compile against Linux version: {}.{}.{}. ",
                        user_selected_linux_feature.unwrap(),
                        major.unwrap(),
                        patch_level.unwrap(),
                        sublevel.unwrap()
                    );
                    println!(
                        "cargo:warning=\tTo set another linux header file location, run `LINUX_HEADERS_PATH=/path/to/directory cargo build --features={}`",
                        user_selected_linux_feature.unwrap()
                    )
                }
            } else {
                // select default features based on parsed linux version
                let major_five_patch_levels = [4, 5, 7, 8, 11, 12, 13, 16];
                let major_six_patch_levels = [0, 3];
                if let Some(mj) = major {
                    match mj {
                        "5" => {
                            if let Some(pl) = patch_level {
                                major_five_patch_levels.map(|v| {
                                    if pl.parse::<usize>().unwrap() >= v {
                                        println!("cargo:rustc-cfg=feature=\"linux-5.{}\"", v)
                                    }
                                });
                            }
                        }
                        "6" => {
                            major_five_patch_levels
                                .map(|x| println!("cargo:rustc-cfg=feature=\"linux-5.{}\"", x));

                            if let Some(pl) = patch_level {
                                major_six_patch_levels.map(|x| {
                                    if pl.parse::<usize>().unwrap() >= x {
                                        println!("cargo:rustc-cfg=feature=\"linux-6.{}\"", x)
                                    }
                                });
                            }
                        }
                        u => panic!("unknown Linux major verion {}", u),
                    };
                }
            }

            let bindings_output_path = "src/syscall/bindings/bindgen.rs";

            let builder = bindgen::Builder::default()
                .derive_default(true)
                .generate_comments(false)
                .prepend_enum_name(false)
                .header("wrapper.h");

            if !linux_headers_path.eq("/usr/include") {
                builder.clang_arg(format!("-I{}", linux_headers_path))
            } else {
                builder
            }
            .generate()
            .unwrap_or_else(|e| panic!("Failed to generate bindings: {}", e))
            .write_to_file(bindings_output_path)
            .unwrap_or_else(|e| panic!("Failed to write {}: {}", bindings_output_path, e));
        }

        tos => panic!("unknown target os {:?}!", tos),
    }
}
