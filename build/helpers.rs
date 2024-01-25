use crate::consts::IOCTLS;
use std::fs;

/// Parse `LINUX_VERSION_CODE` of `linux/version.h` to (major, patch_level, sub_level)
pub fn parse_linux_version_h(path: &str) -> (usize, usize, usize) {
    let first_line = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read {}", path))
        .lines()
        .next()
        .unwrap_or_else(|| panic!("No lines in {}", path))
        .to_string();
    let linux_version_code = first_line
        .split(' ')
        .nth(2)
        .unwrap_or_else(|| panic!("Invalid line {}", first_line))
        .to_string();
    let linux_version_code = linux_version_code.parse::<usize>().unwrap_or_else(|e| {
        panic!(
            "Invalid LINUX_VERSION_CODE `{}` ({})",
            linux_version_code, e
        )
    });

    let major = linux_version_code >> 16;
    let patch_lv = (linux_version_code & 65535) >> 8;
    let sub_lv = linux_version_code & 255;
    (major, patch_lv, sub_lv)
}

pub fn bindgen(linux_headers_path: &str, enabled_feature_versions: &[(usize, usize)]) {
    let bindings_output_path = "src/syscall/bindings/bindgen.rs";
    // Try delete `bindgen.rs` for every build
    let _ = fs::remove_file(bindings_output_path);

    let header_contents = {
        let include_linux_bpf_h = if enabled_feature_versions.contains(&(5, 1)) {
            "#include <linux/bpf.h>"
        } else {
            ""
        };
        let enum_entries = enabled_feature_versions
            .iter()
            .fold(String::new(), |mut acc, it| {
                IOCTLS.iter().try_for_each(|(m, p, str)| {
                    if *it == (*m, *p) {
                        acc.push_str(str);
                        None
                    } else {
                        Some(())
                    }
                });
                acc
            });
        format!(
            "
            #include <asm/unistd.h>
            #include <linux/hw_breakpoint.h>
            #include <linux/perf_event.h>
            {}

            enum perf_event_ioctls {{
                PERF_EVENT_IOCTL_ENABLE     = PERF_EVENT_IOC_ENABLE,
                PERF_EVENT_IOCTL_DISABLE    = PERF_EVENT_IOC_DISABLE,
                PERF_EVENT_IOCTL_REFRESH    = PERF_EVENT_IOC_REFRESH,
                PERF_EVENT_IOCTL_RESET      = PERF_EVENT_IOC_RESET,
                PERF_EVENT_IOCTL_PERIOD     = PERF_EVENT_IOC_PERIOD,
                PERF_EVENT_IOCTL_SET_OUTPUT = PERF_EVENT_IOC_SET_OUTPUT,
                PERF_EVENT_IOCTL_SET_FILTER = PERF_EVENT_IOC_SET_FILTER,
                {}
            }};",
            include_linux_bpf_h, enum_entries,
        )
    };

    let builder = bindgen::Builder::default()
        .derive_default(true)
        .generate_comments(false)
        .prepend_enum_name(false)
        .header_contents("wrapper.h", &header_contents);

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
