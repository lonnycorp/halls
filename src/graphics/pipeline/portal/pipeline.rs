use crate::graphics::model::model_layout;

use super::bind_group::{config_bind_group_layout_create, texture_bind_group_layout_create};
use super::constant::PUSH_CONSTANT_RANGE;

const SHADER_PATH: &str = "shader/portal.wgsl";

pub fn pipeline_portal_create(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Portal Shader"),
        source: wgpu::ShaderSource::Wgsl(
            std::str::from_utf8(crate::ASSET.get_file(SHADER_PATH).unwrap().contents())
                .unwrap()
                .into(),
        ),
    });

    let texture_layout = texture_bind_group_layout_create(device);
    let config_layout = config_bind_group_layout_create(device);

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Portal Pipeline Layout"),
        bind_group_layouts: &[&texture_layout, &config_layout],
        push_constant_ranges: &[PUSH_CONSTANT_RANGE],
    });

    return device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Portal Pipeline"),
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
                blend: Some(wgpu::BlendState::REPLACE),
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
            unclipped_depth: true,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    });
}
