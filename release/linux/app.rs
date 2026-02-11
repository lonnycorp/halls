use std::fs;
use std::path::Path;
use std::process::Command;

use crate::artifact::Artifact;
use crate::read_version;
use crate::{set_executable, APP_NAME, DESCRIPTION, LINUX_TARGET};

pub fn package_appimage() {
    let iconset_dir = Path::new("dist/Halls.iconset");
    let version = read_version();

    let tar_path = Path::new("dist").join(Artifact::LinuxBinaryTarGz.file_name(&version));
    let status = Command::new("tar")
        .arg("czf")
        .arg(&tar_path)
        .arg("-C")
        .arg(format!("target/{LINUX_TARGET}/release"))
        .arg("halls")
        .status()
        .unwrap();
    if !status.success() {
        panic!("tar failed");
    }

    let appdir = Path::new("dist").join(format!("{APP_NAME}.AppDir"));
    fs::create_dir_all(appdir.join("usr/bin")).unwrap();
    fs::create_dir_all(appdir.join("usr/share/applications")).unwrap();
    fs::create_dir_all(appdir.join("usr/share/icons/hicolor/256x256/apps")).unwrap();

    let binary_src = Path::new("target")
        .join(LINUX_TARGET)
        .join("release")
        .join("halls");
    fs::copy(binary_src, appdir.join("usr/bin/halls")).unwrap();

    fs::copy(
        iconset_dir.join("icon_256x256.png"),
        appdir.join("usr/share/icons/hicolor/256x256/apps/halls.png"),
    )
    .unwrap();

    let desktop_template = fs::read_to_string("asset/build/halls.desktop.template").unwrap();
    let desktop_entry = desktop_template
        .replace("{APP_NAME}", APP_NAME)
        .replace("{DESCRIPTION}", DESCRIPTION);
    fs::write(
        appdir.join("usr/share/applications/halls.desktop"),
        &desktop_entry,
    )
    .unwrap();

    let apprun = r#"#!/usr/bin/env sh
HERE="$(dirname "$(readlink -f "$0")")"
exec "$HERE/usr/bin/halls" "$@"
"#;
    fs::write(appdir.join("AppRun"), apprun).unwrap();
    set_executable(&appdir.join("AppRun"));

    fs::write(appdir.join("halls.desktop"), &desktop_entry).unwrap();
    fs::copy(
        iconset_dir.join("icon_256x256.png"),
        appdir.join("halls.png"),
    )
    .unwrap();

    let appimagetool = Path::new("dist/appimagetool.AppImage");
    let status = Command::new("wget")
        .arg("-O")
        .arg(appimagetool)
        .arg("https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage")
        .status()
        .unwrap();
    if !status.success() {
        panic!("wget failed");
    }
    set_executable(appimagetool);

    let appimage_path = Path::new("dist").join(Artifact::LinuxAppImage.file_name(&version));
    let status = Command::new(appimagetool)
        .env("ARCH", "x86_64")
        .arg(&appdir)
        .arg(&appimage_path)
        .status()
        .unwrap();
    if !status.success() {
        panic!("appimagetool failed");
    }
}
