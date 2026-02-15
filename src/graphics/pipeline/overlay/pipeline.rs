use crate::graphics::model::model_layout;

use super::bind_group::{config_bind_group_layout_create, texture_bind_group_layout_create};

const SHADER_PATH: &str = "shader/overlay.wgsl";

pub fn pipeline_overlay_create(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Overlay Shader"),
        source: wgpu::ShaderSource::Wgsl(
            std::str::from_utf8(crate::ASSET.get_file(SHADER_PATH).unwrap().contents())
                .unwrap()
                .into(),
        ),
    });

    let texture_layout = texture_bind_group_layout_create(device);
    let config_layout = config_bind_group_layout_create(device);

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Overlay Pipeline Layout"),
        bind_group_layouts: &[&texture_layout, &config_layout],
        push_constant_ranges: &[],
    });

    return device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Overlay Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[model_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    });
}
