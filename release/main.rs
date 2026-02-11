mod artifact;
mod icon;
mod linux;
mod mac;
mod win;

use std::fs;
use std::path::Path;
use std::process::Command;

pub(crate) const APP_NAME: &str = "Halls";
pub(crate) const BUNDLE_ID: &str = "com.lonnycorp.halls";
pub(crate) const DESCRIPTION: &str =
    "First-person exploration game of internet-hosted 3D spaces linked by URL portals.";
pub(crate) const LINUX_TARGET: &str = "x86_64-unknown-linux-gnu";
pub(crate) const MAC_TARGET: &str = "aarch64-apple-darwin";
pub(crate) const WINDOWS_TARGET: &str = "x86_64-pc-windows-msvc";

fn main() {
    clean_dist();

    if cfg!(target_os = "macos") {
        icon::render_iconset("dist/Halls.iconset", mac::MAC_ICON_VARIANTS);
        cargo_build(MAC_TARGET);
        mac::package();
        return;
    }

    if cfg!(target_os = "linux") {
        icon::render_iconset("dist/Halls.iconset", mac::MAC_ICON_VARIANTS);
        cargo_build(LINUX_TARGET);
        linux::package_appimage();
        linux::package_deb();
        return;
    }

    if cfg!(target_os = "windows") {
        icon::render_iconset("dist/Halls.iconset", win::WINDOWS_ICON_VARIANTS);
        win::build_and_package("dist/Halls.iconset");
        return;
    }

    panic!("Unsupported OS");
}

pub(crate) fn clean_dist() {
    let dist = Path::new("dist");
    if dist.exists() {
        fs::remove_dir_all(dist).unwrap();
    }
    fs::create_dir_all(dist).unwrap();
}

pub(crate) fn cargo_build(target: &str) {
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .arg("--target")
        .arg(target);

    let status = cmd.status().unwrap();
    if !status.success() {
        panic!("cargo build failed");
    }
}

pub(crate) fn set_executable(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).unwrap();
        return;
    }

    #[cfg(not(unix))]
    {
        let _ = path;
        return;
    }
}

pub(crate) fn read_version() -> String {
    let contents = fs::read_to_string("Cargo.toml").unwrap();
    for line in contents.lines() {
        if let Some(version) = line.strip_prefix("version = \"") {
            if let Some(value) = version.strip_suffix('"') {
                return value.to_string();
            }
        }
    }
    panic!("version not found in Cargo.toml");
}
