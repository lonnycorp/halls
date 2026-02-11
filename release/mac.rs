use std::fs;
use std::path::Path;
use std::process::Command;

use crate::artifact::Artifact;
use crate::icon::IconSpec;
use crate::{read_version, APP_NAME, BUNDLE_ID, MAC_TARGET};

pub const MAC_ICON_VARIANTS: &[IconSpec] = &[
    IconSpec::Icon16,
    IconSpec::Icon16At2x,
    IconSpec::Icon32,
    IconSpec::Icon32At2x,
    IconSpec::Icon128,
    IconSpec::Icon128At2x,
    IconSpec::Icon256,
    IconSpec::Icon256At2x,
    IconSpec::Icon512,
    IconSpec::Icon512At2x,
];

pub fn package() {
    let version = read_version();
    let iconset_dir = Path::new("dist/Halls.iconset");
    let icns_path = Path::new("dist/Halls.icns");

    let status = Command::new("iconutil")
        .arg("--convert")
        .arg("icns")
        .arg(iconset_dir)
        .arg("--output")
        .arg(icns_path)
        .status()
        .unwrap();
    if !status.success() {
        panic!("iconutil failed");
    }

    let app_dir = Path::new("dist").join(format!("{APP_NAME}.app"));
    let contents_dir = app_dir.join("Contents");
    let macos_dir = contents_dir.join("MacOS");
    let resources_dir = contents_dir.join("Resources");

    fs::create_dir_all(&macos_dir).unwrap();
    fs::create_dir_all(&resources_dir).unwrap();

    let binary_src = Path::new("target")
        .join(MAC_TARGET)
        .join("release")
        .join("halls");
    let binary_archive_path = Path::new("dist").join(Artifact::MacBinaryTarGz.file_name(&version));
    let status = Command::new("tar")
        .arg("czf")
        .arg(&binary_archive_path)
        .arg("-C")
        .arg(format!("target/{MAC_TARGET}/release"))
        .arg("halls")
        .status()
        .unwrap();
    if !status.success() {
        panic!("tar failed");
    }

    let binary_dst = macos_dir.join("halls");
    fs::copy(&binary_src, &binary_dst).unwrap();
    fs::copy(icns_path, resources_dir.join("halls.icns")).unwrap();

    let plist_template = fs::read_to_string("asset/build/plist.template.xml").unwrap();
    let plist = plist_template
        .replace("{APP_NAME}", APP_NAME)
        .replace("{BUNDLE_ID}", BUNDLE_ID)
        .replace("{version}", &version);

    fs::write(contents_dir.join("Info.plist"), plist).unwrap();

    let zip_path = Path::new("dist").join(Artifact::MacAppZip.file_name(&version));
    let status = Command::new("ditto")
        .arg("-c")
        .arg("-k")
        .arg("--sequesterRsrc")
        .arg("--keepParent")
        .arg(&app_dir)
        .arg(&zip_path)
        .status()
        .unwrap();
    if !status.success() {
        panic!("ditto failed");
    }
}
