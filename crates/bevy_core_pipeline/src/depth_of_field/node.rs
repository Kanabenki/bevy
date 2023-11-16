use std::sync::Mutex;

use bevy_ecs::{prelude::*, query::QueryItem};
use bevy_render::{
    render_graph::{NodeRunError, RenderGraphContext, ViewNode},
    render_resource::{
        BindGroup, BindGroupEntries, LoadOp, Operations, PipelineCache, RenderPassColorAttachment,
        RenderPassDescriptor, TextureViewId,
    },
    renderer::RenderContext,
    view::ViewTarget,
};

use super::{DepthOfField, DepthOfFieldPipeline, ViewDepthOfFieldPipeline};

#[derive(Default)]
pub struct DepthOfFieldNode {
    cached_bind_group: Mutex<Option<(TextureViewId, BindGroup)>>,
}

impl ViewNode for DepthOfFieldNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static DepthOfField,
        &'static ViewDepthOfFieldPipeline,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (target, _dof, view_dof_pipeline): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        let depth_of_field_pipeline = world.resource::<DepthOfFieldPipeline>();

        let pipeline = match pipeline_cache.get_render_pipeline(view_dof_pipeline.0) {
            Some(pipeline) => pipeline,
            None => return Ok(()),
        };

        let post_process = target.post_process_write();
        let source = post_process.source;
        let destination = post_process.destination;

        let mut cached_bind_group = self.cached_bind_group.lock().unwrap();
        let bind_group = match &mut *cached_bind_group {
            Some((texture_id, bind_group)) if source.id() == *texture_id => bind_group,
            cached_bind_group => {
                let bind_group = render_context.render_device().create_bind_group(
                    None,
                    &depth_of_field_pipeline.bind_group_layout,
                    &BindGroupEntries::sequential((source, &depth_of_field_pipeline.sampler)),
                );

                let (_, bind_group) = cached_bind_group.insert((source.id(), bind_group));
                bind_group
            }
        };

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("depth_of_field_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: destination,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Default::default()), // TODO shouldn't need to be cleared
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
