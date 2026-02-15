mod array;
mod depth;
mod sampler;

pub use array::{
    texture_array_bind_group_layout_entry, texture_array_binding_array_bind_group_entry,
    texture_array_binding_array_bind_group_layout_entry, TextureArray,
};
pub use depth::TextureDepth;
pub use sampler::{bind_group_layout_entry as sampler_bind_group_layout_entry, Sampler};
