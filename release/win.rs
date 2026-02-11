use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

use ico::{IconDir, IconDirEntry, IconImage, ResourceType};

use crate::artifact::Artifact;
use crate::icon::IconSpec;
use crate::read_version;
use crate::WINDOWS_TARGET;

pub const WINDOWS_ICON_VARIANTS: &[IconSpec] = &[
    IconSpec::Icon16,
    IconSpec::Icon32,
    IconSpec::Icon48,
    IconSpec::Icon32At2x,
    IconSpec::Icon128,
    IconSpec::Icon256,
];

pub fn build_and_package(iconset_dir: &str) {
    let ico_path = Path::new("dist").join("halls.ico");
    render_ico(iconset_dir, &ico_path);
    build_with_icon(&ico_path);
    package();
    return;
}

fn render_ico(iconset_dir: &str, ico_path: &Path) {
    let mut icon_dir = IconDir::new(ResourceType::Icon);
    let iconset_path = Path::new(iconset_dir);

    for spec in WINDOWS_ICON_VARIANTS {
        let png_path = spec.path(iconset_path);
        let image = image::open(&png_path).unwrap().into_rgba8();
        let icon_image = IconImage::from_rgba_data(spec.width(), spec.height(), image.into_raw());
        let entry = IconDirEntry::encode(&icon_image).unwrap();
        icon_dir.add_entry(entry);
    }

    let mut file = File::create(ico_path).unwrap();
    icon_dir.write(&mut file).unwrap();
    return;
}

fn build_with_icon(ico_path: &Path) {
    let ico_arg = format!("/WIN32ICON:\"{}\"", windows_path(ico_path));
    let mut cmd = Command::new("cargo");
    cmd.arg("rustc")
        .arg("--release")
        .arg("--target")
        .arg(WINDOWS_TARGET)
        .arg("--bin")
        .arg("halls")
        .arg("--")
        .arg("-C")
        .arg(format!("link-arg={}", ico_arg));

    let status = cmd.status().unwrap();
    if !status.success() {
        panic!("cargo rustc failed");
    }
    return;
}

fn windows_path(path: &Path) -> String {
    let absolute = path.canonicalize().unwrap_or_else(|_| PathBuf::from(path));
    let text = absolute.to_string_lossy().replace('/', "\\");
    return text;
}

fn package() {
    let version = read_version();
    let zip_path = Path::new("dist").join(Artifact::WindowsZip.file_name(&version));
    let exe_path = Path::new("target")
        .join(WINDOWS_TARGET)
        .join("release")
        .join("halls.exe");

    let status = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(format!(
            "Compress-Archive -Path \"{}\" -DestinationPath \"{}\"",
            exe_path.display(),
            zip_path.display()
        ))
        .status()
        .unwrap();
    if !status.success() {
        panic!("Compress-Archive failed");
    }
    return;
}
