use super::*;

#[test]
fn test_invalid_data_returns_load_error() {
    let result = GLTFMesh::new(&[]);
    assert!(matches!(result, Err(GLTFMeshError::Load)));
}
