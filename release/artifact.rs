pub enum Artifact {
    LinuxBinaryTarGz,
    LinuxAppImage,
    LinuxDeb,
    MacBinaryTarGz,
    MacAppZip,
    WindowsZip,
}

impl Artifact {
    pub fn file_name(&self, version: &str) -> String {
        match self {
            Artifact::LinuxBinaryTarGz => {
                return format!("halls-{version}-linux-amd64.tar.gz");
            }
            Artifact::LinuxAppImage => {
                return format!("halls-{version}-linux-amd64.AppImage");
            }
            Artifact::LinuxDeb => {
                return format!("halls-{version}-linux-amd64.deb");
            }
            Artifact::MacBinaryTarGz => {
                return format!("halls-{version}-macos-arm64.tar.gz");
            }
            Artifact::MacAppZip => {
                return format!("halls-{version}-macos-arm64.app.zip");
            }
            Artifact::WindowsZip => {
                return format!("halls-{version}-windows-amd64.zip");
            }
        }
    }
}
