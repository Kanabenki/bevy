use bevy_app::prelude::*;
use bevy_asset::{load_internal_asset, Handle};
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_render::{
    camera::Camera,
    extract_component::{ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin},
    render_graph::{RenderGraphApp, ViewNodeRunner},
    render_resource::{
        BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
        CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, MultisampleState,
        PipelineCache, PrimitiveState, RenderPipelineDescriptor, Sampler, SamplerBindingType,
        SamplerDescriptor, Shader, ShaderStages, ShaderType, SpecializedRenderPipeline,
        SpecializedRenderPipelines, TextureFormat, TextureSampleType, TextureViewDimension,
    },
    renderer::RenderDevice,
    texture::BevyDefault,
    view::{ExtractedView, ViewTarget},
    Render, RenderApp, RenderSet,
};

use crate::{core_3d, fullscreen_vertex_shader::fullscreen_shader_vertex_state};

mod node;
pub use node::DepthOfFieldNode;

const DEPTH_OF_FIELD_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(10061292498873997756);

pub struct DepthOfFieldPlugin;

impl Plugin for DepthOfFieldPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            DEPTH_OF_FIELD_SHADER_HANDLE,
            "depth_of_field.wgsl",
            Shader::from_wgsl
        );

        app.register_type::<DepthOfField>();
        app.add_plugins((
            ExtractComponentPlugin::<DepthOfField>::default(),
            UniformComponentPlugin::<DepthOfFieldUniforms>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<SpecializedRenderPipelines<DepthOfFieldPipeline>>()
            .add_systems(Render, prepare_dof_pipelines.in_set(RenderSet::Prepare))
            .add_render_graph_node::<ViewNodeRunner<DepthOfFieldNode>>(
                core_3d::CORE_3D,
                core_3d::graph::node::DEPTH_OF_FIELD,
            )
            .add_render_graph_edges(
                core_3d::CORE_3D,
                &[
                    core_3d::graph::node::TONEMAPPING,
                    core_3d::graph::node::DEPTH_OF_FIELD,
                    core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<DepthOfFieldPipeline>();
    }
}

#[derive(Debug, Clone, Copy, Reflect, Default)]
pub struct DepthOfFieldLayer {
    pub distance: f32,
    pub transition: f32,
}

#[derive(Debug, Clone, Copy, Reflect, ExtractComponent, Component, Default)]
#[extract_component_filter(With<Camera>)]
pub struct DepthOfField {
    pub near: Option<DepthOfFieldLayer>,
    pub far: Option<DepthOfFieldLayer>,
}

#[derive(Component, ShaderType, Clone)]
struct DepthOfFieldUniforms {
    pub near_max: f32,
    pub near_min: f32,
    pub far_min: f32,
    pub far_max: f32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct DepthOfFieldPipelineKey {
    texture_format: TextureFormat,
}

#[derive(Component)]
pub struct ViewDepthOfFieldPipeline(CachedRenderPipelineId);

pub fn prepare_dof_pipelines(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    mut pipelines: ResMut<SpecializedRenderPipelines<DepthOfFieldPipeline>>,
    fxaa_pipeline: Res<DepthOfFieldPipeline>,
    views: Query<(Entity, &ExtractedView, &DepthOfField)>,
) {
    for (entity, view, _dof) in &views {
        let pipeline_id = pipelines.specialize(
            &pipeline_cache,
            &fxaa_pipeline,
            DepthOfFieldPipelineKey {
                texture_format: if view.hdr {
                    ViewTarget::TEXTURE_FORMAT_HDR
                } else {
                    TextureFormat::bevy_default()
                },
            },
        );

        commands
            .entity(entity)
            .insert(ViewDepthOfFieldPipeline(pipeline_id));
    }
}

#[derive(Resource)]
pub struct DepthOfFieldPipeline {
    bind_group_layout: BindGroupLayout,
    sampler: Sampler,
}

impl FromWorld for DepthOfFieldPipeline {
    fn from_world(render_world: &mut World) -> Self {
        let render_device = render_world.resource::<RenderDevice>();
        let bind_group_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("depth_of_field_bind_group_layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                        count: None,
                    },
                ],
            });

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        Self {
            bind_group_layout,
            sampler,
        }
    }
}

impl SpecializedRenderPipeline for DepthOfFieldPipeline {
    type Key = DepthOfFieldPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("depth_of_field_pipeline".into()),
            layout: vec![self.bind_group_layout.clone()],
            vertex: fullscreen_shader_vertex_state(),
            fragment: Some(FragmentState {
                shader: DEPTH_OF_FIELD_SHADER_HANDLE,
                shader_defs: vec![],
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: key.texture_format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: Vec::new(),
        }
    }
}
