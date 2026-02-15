mod mesh;
mod vertex;

#[cfg(test)]
mod test;

pub use mesh::GLTFMesh;
#[cfg(test)]
pub use mesh::GLTFMeshError;
pub use vertex::GLTFVertex;
