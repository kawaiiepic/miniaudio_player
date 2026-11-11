
fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_arch == "wasm32" {
        return;
    }

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let is_apple = target_os == "macos" || target_os == "ios";

    let mut build = cc::Build::new();

    build
        .file("c_src/miniaudio_device_bridge.c")
        .include("c_src")
        .warnings(false)
        .opt_level(3)
        .flag_if_supported("-ffunction-sections")
        .flag_if_supported("-fdata-sections")
        .flag_if_supported("-fvisibility=hidden");

    // Apple audio backends require Objective-C compilation.
    if is_apple {
        build.flag("-x").flag("objective-c");
    }

    match target_os.as_str() {
        "android" => {
            println!("cargo:rustc-link-lib=OpenSLES");
            println!("cargo:rustc-link-lib=log");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=m");
            println!("cargo:rustc-link-lib=dl");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=ole32");
            println!("cargo:rustc-link-lib=winmm");
            println!("cargo:rustc-link-lib=advapi32");
        }
        "macos" | "ios" => {
            for framework in [
                "Foundation",
                "CoreFoundation",
                "CoreAudio",
                "AudioToolbox",
                "AVFoundation",
            ] {
                println!("cargo:rustc-link-lib=framework={framework}");
            }
        }
        _ => {}
    }

    build.compile("miniaudio_bridge");

    println!("cargo:rerun-if-changed=c_src/miniaudio_device_bridge.c");
    println!("cargo:rerun-if-changed=c_src/miniaudio_device_bridge.h");
    println!("cargo:rerun-if-changed=c_src/miniaudio.h");
}
