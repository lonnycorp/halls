mod config;
mod texture;

pub(super) use config::create_bind_group_layout as create_bind_group_layout_config;
pub(super) use texture::create_bind_group_layout as create_bind_group_layout_texture;

pub use config::PipelinePortalBindGroupConfig;
pub use texture::PipelinePortalBindGroupTexture;
