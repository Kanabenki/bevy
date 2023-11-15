use bevy_app::prelude::*;
use bevy_asset::{load_internal_asset, Handle};
use bevy_ecs::{prelude::*, query::QueryItem};
use bevy_reflect::Reflect;
use bevy_render::{
    render_graph::{NodeRunError, RenderGraphApp, RenderGraphContext, ViewNode, ViewNodeRunner},
    render_resource::{
        BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
        BufferBindingType, Sampler, SamplerBindingType, SamplerDescriptor, Shader, ShaderStages,
        ShaderType, TextureSampleType, TextureViewDimension,
    },
    renderer::{RenderContext, RenderDevice},
    view::ViewUniform,
    RenderApp,
};

use crate::core_3d::{self, CORE_3D};

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

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<DepthOfFieldNode>>(
                CORE_3D,
                core_3d::graph::node::DEPTH_OF_FIELD,
            )
            .add_render_graph_edges(
                CORE_3D,
                &[
                    core_3d::graph::node::TONEMAPPING,
                    core_3d::graph::node::DEPTH_OF_FIELD,
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
pub struct DepthOfFieldLayerSettings {
    pub distance: f32,
    pub transition: f32,
}

#[derive(Debug, Clone, Copy, Reflect, Default)]
pub struct DepthOfField {
    pub near: Option<DepthOfFieldLayerSettings>,
    pub far: Option<DepthOfFieldLayerSettings>,
}

struct DepthOfFieldUniforms {
    pub near_max: f32,
    pub near_min: f32,
    pub far_min: f32,
    pub far_max: f32,
}

#[derive(Default)]
struct DepthOfFieldNode;

impl ViewNode for DepthOfFieldNode {
    type ViewQuery = ();

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        _render_context: &mut RenderContext,
        _view_query: QueryItem<Self::ViewQuery>,
        _world: &World,
    ) -> Result<(), NodeRunError> {
        todo!()
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
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: true,
                            min_binding_size: Some(ViewUniform::min_size()),
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
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
