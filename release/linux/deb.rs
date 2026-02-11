use std::fs;
use std::path::Path;
use std::process::Command;

use crate::artifact::Artifact;
use crate::{read_version, APP_NAME, DESCRIPTION, LINUX_TARGET};

pub fn package_deb() {
    let iconset_dir = Path::new("dist/Halls.iconset");
    let version = read_version();
    let deb_dir = Path::new("dist/deb");

    fs::create_dir_all(deb_dir.join("DEBIAN")).unwrap();
    fs::create_dir_all(deb_dir.join("usr/bin")).unwrap();
    fs::create_dir_all(deb_dir.join("usr/share/applications")).unwrap();
    fs::create_dir_all(deb_dir.join("usr/share/icons/hicolor/256x256/apps")).unwrap();

    fs::copy(
        Path::new("target")
            .join(LINUX_TARGET)
            .join("release")
            .join("halls"),
        deb_dir.join("usr/bin/halls"),
    )
    .unwrap();

    fs::copy(
        iconset_dir.join("icon_256x256.png"),
        deb_dir.join("usr/share/icons/hicolor/256x256/apps/halls.png"),
    )
    .unwrap();

    let desktop_template = fs::read_to_string("asset/build/halls.desktop.template").unwrap();
    let desktop_entry = desktop_template
        .replace("{APP_NAME}", APP_NAME)
        .replace("{DESCRIPTION}", DESCRIPTION);
    fs::write(
        deb_dir.join("usr/share/applications/halls.desktop"),
        &desktop_entry,
    )
    .unwrap();

    let control_template = fs::read_to_string("asset/build/deb.control.template").unwrap();
    let control = control_template
        .replace("{version}", &version)
        .replace("{DESCRIPTION}", DESCRIPTION);
    fs::write(deb_dir.join("DEBIAN/control"), control).unwrap();

    let deb_path = Path::new("dist").join(Artifact::LinuxDeb.file_name(&version));
    let status = Command::new("dpkg-deb")
        .arg("--build")
        .arg(deb_dir)
        .arg(&deb_path)
        .status()
        .unwrap();
    if !status.success() {
        panic!("dpkg-deb failed");
    }
}
