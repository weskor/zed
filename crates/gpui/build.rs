#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]

fn main() {
    println!("cargo::rustc-check-cfg=cfg(gles)");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "windows" {
        #[cfg(feature = "windows-manifest")]
        embed_resource();
    }
}

#[cfg(feature = "windows-manifest")]
fn embed_resource() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest = manifest_dir.join("resources/windows/gpui.manifest.xml");
    let rc_file = manifest_dir.join("resources/windows/gpui.rc");
    println!("cargo:rerun-if-changed={}", manifest.display());
    println!("cargo:rerun-if-changed={}", rc_file.display());
    let resource_path = manifest.to_string_lossy().replace('\\', "\\\\");
    let resource = std::fs::read_to_string(rc_file)
        .unwrap()
        .replace("resources/windows/gpui.manifest.xml", &resource_path);
    let generated_rc = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("gpui.rc");
    std::fs::write(&generated_rc, resource).unwrap();
    embed_resource::compile(&generated_rc, embed_resource::NONE)
        .manifest_required()
        .unwrap();
}
