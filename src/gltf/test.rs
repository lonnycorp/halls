use super::*;

const WHITE: [u8; 4] = [255, 255, 255, 255];

#[test]
fn test_invalid_data_returns_load_error() {
    let result = GLTFMesh::from_bytes(&[]);
    assert!(matches!(result, Err(GLTFMeshError::Load)));
}

#[test]
fn test_vertex_without_color_returns_none() {
    let mesh = GLTFMesh::new(
        vec![
            0.0, 0.0, 0.0, //
            1.0, 0.0, 0.0, //
            0.0, 1.0, 0.0,
        ],
        vec![0, 1, 2],
        None,
        None,
        None,
    );

    let vertex = mesh.vertex(0);
    assert_eq!(vertex.color, None);
}

#[test]
fn test_vertex_with_color_returns_some() {
    let mesh = GLTFMesh::new(
        vec![
            0.0, 0.0, 0.0, //
            1.0, 0.0, 0.0, //
            0.0, 1.0, 0.0,
        ],
        vec![2, 1, 0],
        None,
        None,
        Some(vec![
            10, 20, 30, 255, //
            40, 50, 60, 128, //
            70, 80, 90, 64,
        ]),
    );

    let vertex = mesh.vertex(0);
    assert_eq!(vertex.color, Some([70, 80, 90, 64]));
}

#[test]
fn test_vertex_default_model_color_is_white() {
    let mesh = GLTFMesh::new(
        vec![
            0.0, 0.0, 0.0, //
            1.0, 0.0, 0.0, //
            0.0, 1.0, 0.0,
        ],
        vec![0, 1, 2],
        None,
        None,
        None,
    );

    let vertex = mesh.vertex(0);
    let mut buffer = crate::graphics::model::ModelBuffer::new();
    vertex.write_to_model_buffer(&mut buffer);

    assert_eq!(buffer.vertices()[0].color, WHITE);
}

#[test]
fn test_new_mesh_has_no_material_names() {
    let mesh = GLTFMesh::new(
        vec![
            0.0, 0.0, 0.0, //
            1.0, 0.0, 0.0, //
            0.0, 1.0, 0.0,
        ],
        vec![0, 1, 2],
        None,
        None,
        None,
    );

    assert_eq!(mesh.materials().len(), 0);
}
