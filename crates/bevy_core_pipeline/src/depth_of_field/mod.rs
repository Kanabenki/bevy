use bevy_app::prelude::*;
use bevy_asset::{load_internal_asset, Handle};
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_render::{
    render_resource::{BindGroupLayout, Shader},
    RenderApp,
};

const DEPTH_OF_FIELD_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(0);

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
    }

    fn finish(&self, app: &mut App) {
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.init_resource::<DepthOfFieldPipeline>();
        }
    }
}

#[derive(Debug, Clone, Copy, Reflect, Default)]
struct DepthOfField;

#[derive(Resource)]
pub struct DepthOfFieldPipeline {
    _bind_group_layout: BindGroupLayout,
}

impl Default for DepthOfFieldPipeline {
    fn default() -> Self {
        todo!()
    }
}
