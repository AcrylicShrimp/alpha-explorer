use crate::{
    component::*,
    engine::use_context,
    gfx::{
        low::{DeviceAllocation, HostAllocation},
        *,
    },
    handles::{BufferHandle, PipelineHandle},
    structure::{Mat33, Vec2},
};
use fontdue::layout::{HorizontalAlign, VerticalAlign};
use rayon::slice::ParallelSliceMut;
use specs::prelude::*;
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    mem::{size_of, size_of_val},
    num::NonZeroU64,
    sync::Arc,
};
use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType,
    Buffer, BufferAddress, BufferBinding, BufferBindingType, Color, LoadOp, Operations,
    RenderPassColorAttachment, RenderPassDescriptor, ShaderStages,
};

pub struct RenderSystem {
    camera_bind_group_layout: BindGroupLayout,
    sprite_per_vertex_buffer: BufferHandle,
}

impl RenderSystem {
    pub fn new(render_mgr: &mut RenderManager) -> Self {
        render_mgr.register_pipeline_factory::<GlyphRenderPipelineFactoryProvider>();
        render_mgr.register_pipeline_factory::<SpriteRenderPipelineFactoryProvider>();

        let camera_bind_group_layout =
            render_mgr.create_bind_group_layout(&[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: Some(
                        NonZeroU64::new((size_of::<f32>() * 12) as u64).unwrap(),
                    ),
                },
                count: None,
            }]);

        let sprite_per_vertex_buffer = render_mgr.create_vertex_buffer(&[
            1f32, 1f32, //
            1f32, 0f32, //
            //
            0f32, 0f32, //
            0f32, 1f32, //
            //
            1f32, 0f32, //
            1f32, 1f32, //
            //
            //
            //
            1f32, 1f32, //
            1f32, 0f32, //
            //
            0f32, 1f32, //
            0f32, 0f32, //
            //
            0f32, 0f32, //
            0f32, 1f32, //
        ]);

        // let glyph_buffer = Buffer::from_slice(&[
        //     1f32, 1f32, //
        //     1f32, 0f32, //
        //     //
        //     0f32, 0f32, //
        //     0f32, 1f32, //
        //     //
        //     1f32, 0f32, //
        //     1f32, 1f32, //
        //     //
        //     //
        //     //
        //     1f32, 1f32, //
        //     1f32, 0f32, //
        //     //
        //     0f32, 1f32, //
        //     0f32, 0f32, //
        //     //
        //     0f32, 0f32, //
        //     0f32, 1f32, //
        // ]);
        // let tilemap_sprite_buffer = Buffer::from_slice(&[
        //     1f32, 1f32, //
        //     1f32, 0f32, //
        //     //
        //     0f32, 0f32, //
        //     0f32, 1f32, //
        //     //
        //     1f32, 0f32, //
        //     1f32, 1f32, //
        //     //
        //     //
        //     //
        //     1f32, 1f32, //
        //     1f32, 0f32, //
        //     //
        //     0f32, 1f32, //
        //     0f32, 0f32, //
        //     //
        //     0f32, 0f32, //
        //     0f32, 1f32, //
        // ]);
        // let alpha_tilemap_back_buffer = Buffer::from_slice(&[
        //     1f32, 1f32, //
        //     //
        //     0f32, 0f32, //
        //     //
        //     1f32, 0f32, //
        //     //
        //     //
        //     //
        //     1f32, 1f32, //
        //     //
        //     0f32, 1f32, //
        //     //
        //     0f32, 0f32, //
        // ]);

        Self {
            // glyph_buffer,
            camera_bind_group_layout,
            sprite_per_vertex_buffer,
            // tilemap_sprite_buffer,
            // alpha_tilemap_back_buffer,
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, GlyphRenderer>,
        ReadStorage<'a, SpriteRenderer>,
        // ReadStorage<'a, TilemapRenderer>,
        // ReadStorage<'a, AlphaTilemapRenderer>,
    );

    fn run(
        &mut self,
        (
            camera,
            transform,
            size,
            glyph_renderer,
            sprite_renderer,
            // tilemap_renderer,
            // alpha_tilemap_renderer,
        ): Self::SystemData,
    ) {
        let context = use_context();
        let mut render_mgr = context.render_mgr_mut();
        let glyph_mgr = context.glyph_mgr();
        let screen_mgr = context.screen_mgr();
        let transform_mgr = context.transform_mgr();

        let (surface_texture, surface_texture_view) = render_mgr.create_render_output();
        // let stencil_texture = render_mgr.stencil_texture();
        let mut encoder = render_mgr.create_encoder();

        let width_half = (screen_mgr.width() * 0.5) as f32;
        let height_half = (screen_mgr.height() * 0.5) as f32;
        let sdf_inset = glyph_mgr.sdf_inset();

        let mut camera_with_transform_buffers = (&camera, &transform)
            .join()
            .map(|(camera, transform)| {
                let world_to_ndc = transform_mgr
                    .transform_world_matrix(transform.index())
                    .inversed()
                    * Mat33::affine_scale(Vec2::new(1f32 / width_half, 1f32 / height_half));
                let matrix_elements = world_to_ndc.elements();
                CameraWithTransformBuffer {
                    camera,
                    transform,
                    transform_buffer: render_mgr.create_single_frame_uniform_buffer(&[
                        matrix_elements[0],
                        matrix_elements[1],
                        matrix_elements[2],
                        0f32, // Padding
                        matrix_elements[3],
                        matrix_elements[4],
                        matrix_elements[5],
                        0f32, // Padding
                        matrix_elements[6],
                        matrix_elements[7],
                        matrix_elements[8],
                        0f32, // Padding
                    ]),
                }
            })
            .collect::<Vec<_>>();
        camera_with_transform_buffers.sort_unstable_by(
            |CameraWithTransformBuffer { camera: lhs, .. },
             CameraWithTransformBuffer { camera: rhs, .. }| lhs.order.cmp(&rhs.order),
        );
        let camera_bind_groups = camera_with_transform_buffers
            .iter()
            .map(
                |CameraWithTransformBuffer {
                     transform_buffer, ..
                 }| {
                    render_mgr.create_bind_group(
                        &self.camera_bind_group_layout,
                        &[BindGroupEntry {
                            binding: 0,
                            resource: BindingResource::Buffer(BufferBinding {
                                buffer: &transform_buffer.buffer(),
                                offset: transform_buffer.offset(),
                                size: Some(NonZeroU64::new(transform_buffer.size()).unwrap()),
                            }),
                        }],
                    )
                },
            )
            .collect::<Vec<_>>();

        let mut render_request_indices = Vec::with_capacity(4 * 1024);
        let mut render_requests = Vec::with_capacity(4 * 1024);

        // TODO: Parallelize here to improve performance.
        for (index, CameraWithTransformBuffer { camera, .. }) in
            camera_with_transform_buffers.iter().enumerate()
        {
            let render_pass_load_ops = match camera.clear_mode {
                ClearMode::None => LoadOp::Load,
                ClearMode::Color => LoadOp::Clear(Color {
                    r: camera.clear_color.r as f64,
                    g: camera.clear_color.g as f64,
                    b: camera.clear_color.b as f64,
                    a: camera.clear_color.a as f64,
                }),
            };
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &surface_texture_view,
                    resolve_target: None,
                    ops: Operations {
                        load: render_pass_load_ops,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
                // depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                //     view: &stencil_texture.view,
                //     depth_ops: None,
                //     stencil_ops: Some(Operations {
                //         load: LoadOp::Clear(0),
                //         store: true,
                //     }),
                // }),
            });

            for (transform, size, renderer) in (&transform, &size, &glyph_renderer).join() {
                if !Layer::has_overlap(camera.layer, renderer.layer) {
                    return;
                }

                let matrix = transform_mgr.transform_world_matrix(transform.index());

                let size = size.size;
                let layout_size = renderer.compute_size();
                let (horizontal_align, vertical_align) = (
                    match renderer.config().horizontal_align {
                        HorizontalAlign::Left => 0f32,
                        HorizontalAlign::Center => 0.5f32,
                        HorizontalAlign::Right => 1f32,
                    },
                    match renderer.config().vertical_align {
                        VerticalAlign::Top => 0f32,
                        VerticalAlign::Middle => 0.5f32,
                        VerticalAlign::Bottom => 1f32,
                    },
                );
                let overflow_offset =
                    Vec2::new((size.width * 0.5) as f32, (size.height * 0.5) as f32);
                let alignment_offset = Vec2::new(
                    (size.width - layout_size.width) * horizontal_align,
                    (size.height - layout_size.height) * vertical_align,
                );
                let offset = alignment_offset - overflow_offset;

                let layout = renderer.layout();
                let glyphs = renderer.glyphs();

                for (position, glyph) in layout.glyphs().iter().zip(glyphs) {
                    let mapping = glyph.sprite().mapping();
                    let texture = glyph.sprite().texture();

                    let font_width_scale = if mapping.width() as usize == 2 * sdf_inset {
                        0f32
                    } else {
                        position.width as f32 / (mapping.width() as usize - 2 * sdf_inset) as f32
                    };
                    let font_height_scale = if mapping.height() as usize == 2 * sdf_inset {
                        0f32
                    } else {
                        position.height as f32 / (mapping.height() as usize - 2 * sdf_inset) as f32
                    };

                    let glyph_width =
                        position.width as f32 + 2f32 * font_width_scale * sdf_inset as f32;
                    let glyph_height =
                        position.height as f32 + 2f32 * font_height_scale * sdf_inset as f32;
                    let matrix_elements = (Mat33::affine_translation(Vec2::new(
                        position.x + offset.x - sdf_inset as f32 * font_width_scale,
                        position.y - offset.y - sdf_inset as f32 * font_height_scale,
                    )) * matrix)
                        .into_elements();

                    let per_instance_buffer_contents = [
                        matrix_elements[0],
                        matrix_elements[1],
                        matrix_elements[2],
                        matrix_elements[3],
                        matrix_elements[4],
                        matrix_elements[5],
                        matrix_elements[6],
                        matrix_elements[7],
                        matrix_elements[8],
                        glyph_width,
                        glyph_height,
                        renderer.color.r,
                        renderer.color.g,
                        renderer.color.b,
                        renderer.color.a,
                        renderer.thickness,
                        renderer.smoothness,
                        (mapping.min().0 as f32) / texture.width as f32,
                        (mapping.min().1 as f32) / texture.height as f32,
                        (mapping.max().0 as f32) / texture.width as f32,
                        (mapping.max().1 as f32) / texture.height as f32,
                    ];
                    let size = size_of_val(&per_instance_buffer_contents);

                    let request = RenderRequest {
                        pipeline: render_mgr
                            .allocate_pipeline::<GlyphRenderPipelineFactoryProvider>(
                                &renderer.shader,
                            ),
                        bind_group: glyph.bind_group(),
                        per_vertex_buffer: &self.sprite_per_vertex_buffer,
                        per_instance_buffer: render_mgr
                            .create_single_frame_vertex_buffer_without_contents(
                                size as BufferAddress,
                            ),
                        per_instance_data: render_mgr
                            .create_single_frame_host_buffer(&per_instance_buffer_contents),
                    };
                    render_request_indices.push(RenderRequestIndex::from_request(
                        render_requests.len() as u32,
                        renderer.order,
                        &request,
                    ));
                    render_requests.push(request);
                }
            }

            for (transform, size, renderer) in (&transform, &size, &sprite_renderer).join() {
                if !Layer::has_overlap(camera.layer, renderer.layer) {
                    return;
                }

                let matrix = transform_mgr.transform_world_matrix(transform.index());
                let matrix_elements = matrix.elements();

                let mapping = renderer.sprite().mapping();
                let texture = renderer.sprite().texture();

                let per_instance_buffer_contents = [
                    matrix_elements[0],
                    matrix_elements[1],
                    matrix_elements[2],
                    matrix_elements[3],
                    matrix_elements[4],
                    matrix_elements[5],
                    matrix_elements[6],
                    matrix_elements[7],
                    matrix_elements[8],
                    size.size.width,
                    size.size.height,
                    renderer.color.r,
                    renderer.color.g,
                    renderer.color.b,
                    renderer.color.a,
                    mapping.x_min as f32 / texture.width as f32,
                    mapping.y_min as f32 / texture.height as f32,
                    mapping.x_max as f32 / texture.width as f32,
                    mapping.y_max as f32 / texture.height as f32,
                ];
                let size = size_of_val(&per_instance_buffer_contents);

                let request = RenderRequest {
                    pipeline: render_mgr
                        .allocate_pipeline::<SpriteRenderPipelineFactoryProvider>(&renderer.shader),
                    bind_group: renderer.bind_group(),
                    per_vertex_buffer: &self.sprite_per_vertex_buffer,
                    per_instance_buffer: render_mgr
                        .create_single_frame_vertex_buffer_without_contents(size as BufferAddress),
                    per_instance_data: render_mgr
                        .create_single_frame_host_buffer(&per_instance_buffer_contents),
                };
                render_request_indices.push(RenderRequestIndex::from_request(
                    render_requests.len() as u32,
                    renderer.order,
                    &request,
                ));
                render_requests.push(request);
            }

            render_pass.set_bind_group(0, &camera_bind_groups[index], &[]);
            render_request_indices.par_sort_unstable();

            let mut index = 0;
            let mut last_pipeline = None;

            // Do dynamic batching and render them.
            while index < render_request_indices.len() {
                let mut instance_count = 1;
                let mut last_request_index = index;

                for additional_request_index in (index + 1)..render_request_indices.len() {
                    let before = &render_request_indices[last_request_index];
                    let after = &render_request_indices[additional_request_index];

                    if !after.can_be_merged(before) {
                        break;
                    }

                    instance_count += 1;
                    last_request_index = additional_request_index
                }

                let request_index = &render_request_indices[index];
                let request = &render_requests[request_index.request_index as usize];

                // Collect per-instance vertex buffers.
                let single_per_instance_buffer_size = request.per_instance_data.size();
                let per_instance_buffer_size = single_per_instance_buffer_size * instance_count;
                let per_instance_buffer = render_mgr
                    .create_single_frame_host_buffer_without_contents(per_instance_buffer_size);

                for batch_count in 0..instance_count {
                    let batched_request_index = &render_request_indices[index + batch_count];
                    let batched_request =
                        &render_requests[batched_request_index.request_index as usize];
                    let offset = single_per_instance_buffer_size * batch_count;
                    per_instance_buffer
                        .copy_from_allocation(&batched_request.per_instance_data, offset);
                }

                render_mgr.write_single_frame_device_buffer_contents(
                    &request.per_instance_buffer,
                    &per_instance_buffer.buffer().borrow()[per_instance_buffer.range()],
                );

                if match last_pipeline {
                    Some(pipeline) => pipeline != &request.pipeline,
                    None => true,
                } {
                    last_pipeline = Some(&request.pipeline);
                    render_pass.set_pipeline(&request.pipeline);
                    render_pass.set_vertex_buffer(0, request.per_vertex_buffer.slice(..));
                }

                render_pass.set_bind_group(1, request.bind_group, &[]);
                render_pass.set_vertex_buffer(
                    1,
                    request
                        .per_instance_buffer
                        .as_slice_instanced(instance_count),
                );
                render_pass.draw(0..6, 0..instance_count as u32);

                index += instance_count;
            }

            // OLD: Below is old.

            // let mut last_pipeline = None;

            // for &RenderRequestIndex { request_index, .. } in &render_request_indices {
            //     let request = &render_requests[request_index as usize];

            //     if match last_pipeline {
            //         Some(pipeline) => pipeline != &request.pipeline,
            //         None => true,
            //     } {
            //         last_pipeline = Some(&request.pipeline);
            //         render_pass.set_pipeline(&request.pipeline);
            //         render_pass.set_vertex_buffer(0, request.per_vertex_buffer.slice(..));
            //     }

            //     render_pass.set_bind_group(1, request.bind_group, &[]);
            //     render_pass.set_vertex_buffer(1, request.per_instance_buffer.as_slice());
            //     render_pass.draw(0..6, 0..1);
            // }

            // let camera_transform_index = camera_transform.index();
            // let world_to_ndc = transform_mgr
            //     .transform_world_matrix(camera_transform_index)
            //     .inversed()
            //     * Mat33::affine_scale(Vec2::new(1f32 / width_half, 1f32 / height_half));
            // let ndc_to_world = Mat33::affine_scale(Vec2::new(width_half, height_half))
            //     * transform_mgr.transform_world_matrix(camera_transform_index);

            // let mut buffers = bump_vec![in &self.extra_bump];
            // let mut renderers = bump_vec![in &self.extra_bump];
            // let sdf_inset = glyph_mgr.sdf_inset();

            // for (transform, size, renderer) in (&transform, &size, &mut glyph_renderer).join() {
            //     if !Layer::has_overlap(camera.layer, renderer.layer) {
            //         return;
            //     }

            //     let size = size.size;
            //     let color = renderer.color;
            //     let thickness = renderer.thickness;
            //     let smoothness = renderer.smoothness;
            //     let layout_size = renderer.compute_size();
            //     let (horizontal_align, vertical_align) = (
            //         match renderer.config().horizontal_align {
            //             HorizontalAlign::Left => 0f32,
            //             HorizontalAlign::Center => 0.5f32,
            //             HorizontalAlign::Right => 1f32,
            //         },
            //         match renderer.config().vertical_align {
            //             VerticalAlign::Top => 0f32,
            //             VerticalAlign::Middle => 0.5f32,
            //             VerticalAlign::Bottom => 1f32,
            //         },
            //     );
            //     let overflow_offset =
            //         Vec2::new((size.width * 0.5) as f32, (size.height * 0.5) as f32);
            //     let alignment_offset = Vec2::new(
            //         (size.width - layout_size.width) * horizontal_align,
            //         (size.height - layout_size.height) * vertical_align,
            //     );
            //     let offset = alignment_offset - overflow_offset;
            //     let matrix = transform_mgr.transform_world_matrix(transform.index());
            //     let mut texture_and_buffers = bump_vec![in &self.extra_bump];

            //     let (font, layout) = renderer.font_and_layout();

            //     for glyph in layout.glyphs() {
            //         let g = glyph_mgr.glyph(font, glyph.key);
            //         let mut buffer = render_mgr.alloc_buffer();

            //         let font_width_scale = if g.mapping.width() as usize == 2 * sdf_inset {
            //             0f32
            //         } else {
            //             glyph.width as f32 / (g.mapping.width() as usize - 2 * sdf_inset) as f32
            //         };
            //         let font_height_scale = if g.mapping.height() as usize == 2 * sdf_inset {
            //             0f32
            //         } else {
            //             glyph.height as f32 / (g.mapping.height() as usize - 2 * sdf_inset) as f32
            //         };

            //         let glyph_width =
            //             glyph.width as f32 + 2f32 * font_width_scale * sdf_inset as f32;
            //         let glyph_height =
            //             glyph.height as f32 + 2f32 * font_height_scale * sdf_inset as f32;
            //         let matrix = (Mat33::affine_translation(Vec2::new(
            //             glyph.x + offset.x - sdf_inset as f32 * font_width_scale,
            //             glyph.y - offset.y - sdf_inset as f32 * font_height_scale,
            //         )) * matrix)
            //             .into_elements();

            //         buffer.replace(&[
            //             matrix[0],
            //             matrix[1],
            //             matrix[2],
            //             matrix[3],
            //             matrix[4],
            //             matrix[5],
            //             matrix[6],
            //             matrix[7],
            //             matrix[8],
            //             glyph_width,
            //             glyph_height,
            //             color.r,
            //             color.g,
            //             color.b,
            //             color.a,
            //             thickness,
            //             smoothness,
            //             (g.mapping.min().0 as f32) / g.texture.width() as f32,
            //             (g.mapping.min().1 as f32) / g.texture.height() as f32,
            //             (g.mapping.max().0 as f32) / g.texture.width() as f32,
            //             (g.mapping.max().1 as f32) / g.texture.height() as f32,
            //         ]);

            //         texture_and_buffers.push((g.texture.handle(), buffer));
            //     }

            //     let shader = &renderer.shader;
            //     let mut r = Renderer::new(&self.renderer_bump);

            //     // TODO: Merge instances that have the same texture to reduce draw calls.

            //     for (texture, buffer) in texture_and_buffers {
            //         r.enqueue(1, 2, RenderMode::Trangles, shader, |req| {
            //             render_mgr.apply_common_shader_input(shader, req);

            //             // TODO: Add shader type checking logic to alert if types have no match.

            //             if let Some(uniform) = shader.uniform("camera") {
            //                 req.uniform_f33(uniform.location, world_to_ndc.elements().clone());
            //             }
            //             if let Some(uniform) = shader.uniform("glyph") {
            //                 req.uniform_texture_raw(uniform.location, texture);
            //             }

            //             if let Some(attribute) = shader.attribute("pos") {
            //                 req.attribute(attribute.location, &self.glyph_buffer, 0, attribute.ty);
            //             }
            //             if let Some(attribute) = shader.attribute("uv") {
            //                 req.attribute(
            //                     attribute.location,
            //                     &self.glyph_buffer,
            //                     (size_of::<f32>() * 2) as _,
            //                     attribute.ty,
            //                 );
            //             }

            //             if let Some(attribute) = shader.attribute("transform") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &buffer,
            //                     0,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = shader.attribute("size") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &buffer,
            //                     (size_of::<f32>() * 9) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = shader.attribute("color") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &buffer,
            //                     (size_of::<f32>() * 11) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = shader.attribute("thickness") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &buffer,
            //                     (size_of::<f32>() * 15) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = shader.attribute("smoothness") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &buffer,
            //                     (size_of::<f32>() * 16) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = shader.attribute("uv_rect") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &buffer,
            //                     (size_of::<f32>() * 17) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //         });
            //         buffers.push(buffer);
            //     }

            //     renderers.push((renderer.order, r));
            // }

            // // for (transform, size, renderer) in (&transform, &size, &nine_patch_renderer).join() {
            // //     if !Layer::has_overlap(camera.layer, renderer.layer) {
            // //         return;
            // //     }

            // //     let size = size.size;
            // //     let matrix = transform_mgr.transform_world_matrix(transform.index());
            // //     let nine_patch = &renderer.nine_patch;

            // //     let left = nine_patch.sprite_lt().width() as f32;
            // //     let right = nine_patch.sprite_rt().width() as f32;
            // //     let center = f32::max(0f32, size.width - left - right);
            // //     let (left, right) = if 0f32 < center {
            // //         (left, right)
            // //     } else {
            // //         let ratio = size.width / (left + right);
            // //         (left * ratio, right * ratio)
            // //     };

            // //     let top = nine_patch.sprite_lt().height() as f32;
            // //     let bottom = nine_patch.sprite_lb().height() as f32;
            // //     let middle = f32::max(0f32, size.height - top - bottom);
            // //     let (top, bottom) = if 0f32 < middle {
            // //         (top, bottom)
            // //     } else {
            // //         let ratio = size.height / (top + bottom);
            // //         (top * ratio, bottom * ratio)
            // //     };

            // //     let mut buffer_data: Vec<f32> = Vec::with_capacity(9 * 19);
            // //     let mut enqueue_patch = |offset_x: f32,
            // //                              offset_y: f32,
            // //                              width: f32,
            // //                              height: f32,
            // //                              sprite: &Sprite| {
            // //         buffer_data.extend(&[
            // //             matrix[0],
            // //             matrix[1],
            // //             matrix[2],
            // //             matrix[3],
            // //             matrix[4],
            // //             matrix[5],
            // //             matrix[6]
            // //                 + matrix[0] * (offset_x - size.width * 0.5f32)
            // //                 + matrix[3] * (offset_y + size.height * 0.5f32),
            // //             matrix[7]
            // //                 + matrix[1] * (offset_x - size.width * 0.5f32)
            // //                 + matrix[4] * (offset_y + size.height * 0.5f32),
            // //             matrix[8],
            // //             width,
            // //             height,
            // //             renderer.color.r,
            // //             renderer.color.g,
            // //             renderer.color.b,
            // //             renderer.color.a,
            // //             (sprite.texel_mapping().min().0 as f32) / sprite.texture().width() as f32,
            // //             (sprite.texel_mapping().min().1 as f32) / sprite.texture().height() as f32,
            // //             (sprite.texel_mapping().max().0 as f32) / sprite.texture().width() as f32,
            // //             (sprite.texel_mapping().max().1 as f32) / sprite.texture().height() as f32,
            // //         ]);
            // //     };

            // //     let mut patch_count = 0;

            // //     if 0f32 < left && 0f32 < top {
            // //         patch_count += 1;
            // //         enqueue_patch(0f32, -top, left, top, &nine_patch.sprite_lt());
            // //     }

            // //     if 0f32 < center && 0f32 < top {
            // //         patch_count += 1;
            // //         enqueue_patch(left, -top, center, top, &nine_patch.sprite_ct());
            // //     }

            // //     if 0f32 < right && 0f32 < top {
            // //         patch_count += 1;
            // //         enqueue_patch(left + center, -top, right, top, &nine_patch.sprite_rt());
            // //     }

            // //     if 0f32 < left && 0f32 < middle {
            // //         patch_count += 1;
            // //         enqueue_patch(0f32, -(top + middle), left, middle, &nine_patch.sprite_lm());
            // //     }

            // //     if 0f32 < center && 0f32 < middle {
            // //         patch_count += 1;
            // //         enqueue_patch(
            // //             left,
            // //             -(top + middle),
            // //             center,
            // //             middle,
            // //             &nine_patch.sprite_cm(),
            // //         );
            // //     }

            // //     if 0f32 < right && 0f32 < middle {
            // //         patch_count += 1;
            // //         enqueue_patch(
            // //             left + center,
            // //             -(top + middle),
            // //             right,
            // //             middle,
            // //             &nine_patch.sprite_rm(),
            // //         );
            // //     }

            // //     if 0f32 < left && 0f32 < bottom {
            // //         patch_count += 1;
            // //         enqueue_patch(
            // //             0f32,
            // //             -(top + middle + bottom),
            // //             left,
            // //             bottom,
            // //             &nine_patch.sprite_lb(),
            // //         );
            // //     }

            // //     if 0f32 < center && 0f32 < bottom {
            // //         patch_count += 1;
            // //         enqueue_patch(
            // //             left,
            // //             -(top + middle + bottom),
            // //             center,
            // //             bottom,
            // //             &nine_patch.sprite_cb(),
            // //         );
            // //     }

            // //     if 0f32 < right && 0f32 < bottom {
            // //         patch_count += 1;
            // //         enqueue_patch(
            // //             left + center,
            // //             -(top + middle + bottom),
            // //             right,
            // //             bottom,
            // //             &nine_patch.sprite_rb(),
            // //         );
            // //     }

            // //     let mut buffer = render_mgr.alloc_buffer();
            // //     buffer.replace(buffer_data.as_slice());

            // //     let shader = &renderer.shader;
            // //     let mut r = Renderer::new(&self.renderer_bump);

            // //     r.enqueue(patch_count, 2, RenderMode::Trangles, shader, |req| {
            // //         render_mgr.apply_common_shader_input(shader, req);

            // //         // TODO: Add shader type checking logic to alert if types have no match.

            // //         if let Some(uniform) = shader.uniform("camera") {
            // //             req.uniform_f33(uniform.location, camera_matrix_inverse);
            // //         }
            // //         if let Some(uniform) = shader.uniform("sprite") {
            // //             req.uniform_texture(uniform.location, nine_patch.texture());
            // //         }

            // //         if let Some(attribute) = shader.attribute("pos") {
            // //             req.attribute(attribute.location, &self.sprite_buffer, 0, attribute.ty);
            // //         }
            // //         if let Some(attribute) = shader.attribute("uv") {
            // //             req.attribute(
            // //                 attribute.location,
            // //                 &self.sprite_buffer,
            // //                 (size_of::<f32>() * 2) as _,
            // //                 attribute.ty,
            // //             );
            // //         }

            // //         if let Some(attribute) = shader.attribute("transform") {
            // //             req.attribute_per_instance(attribute.location, &buffer, 0, attribute.ty);
            // //         }
            // //         if let Some(attribute) = shader.attribute("size") {
            // //             req.attribute_per_instance(
            // //                 attribute.location,
            // //                 &buffer,
            // //                 (size_of::<f32>() * 9) as _,
            // //                 attribute.ty,
            // //             );
            // //         }
            // //         if let Some(attribute) = shader.attribute("color") {
            // //             req.attribute_per_instance(
            // //                 attribute.location,
            // //                 &buffer,
            // //                 (size_of::<f32>() * 11) as _,
            // //                 attribute.ty,
            // //             );
            // //         }
            // //         if let Some(attribute) = shader.attribute("uv_rect") {
            // //             req.attribute_per_instance(
            // //                 attribute.location,
            // //                 &buffer,
            // //                 (size_of::<f32>() * 15) as _,
            // //                 attribute.ty,
            // //             );
            // //         }
            // //     });
            // //     buffers.push(buffer);
            // //     renderers.push((renderer.order, r));
            // // }

            // for (transform, renderer) in (&transform, &tilemap_renderer).join() {
            //     if !Layer::has_overlap(camera.layer, renderer.layer) {
            //         return;
            //     }

            //     let matrix = transform_mgr.transform_world_matrix(transform.index());
            //     let ndc_to_local = ndc_to_world.as_ref() * matrix.inversed();

            //     let aabb_lt = (Vec3::new(-1f32, 1f32, 1f32) * ndc_to_local.as_ref()).to_vec2();
            //     let aabb_lb = (Vec3::new(-1f32, -1f32, 1f32) * ndc_to_local.as_ref()).to_vec2();
            //     let aabb_rt = (Vec3::new(1f32, 1f32, 1f32) * ndc_to_local.as_ref()).to_vec2();
            //     let aabb_rb = (Vec3::new(1f32, -1f32, 1f32) * ndc_to_local.as_ref()).to_vec2();

            //     let aabb_min_x = f32::min(
            //         f32::min(aabb_lt.x, aabb_lb.x),
            //         f32::min(aabb_rt.x, aabb_rb.x),
            //     );
            //     let aabb_max_x = f32::max(
            //         f32::max(aabb_lt.x, aabb_lb.x),
            //         f32::max(aabb_rt.x, aabb_rb.x),
            //     );
            //     let aabb_min_y = f32::min(
            //         f32::min(aabb_lt.y, aabb_lb.y),
            //         f32::min(aabb_rt.y, aabb_rb.y),
            //     );
            //     let aabb_max_y = f32::max(
            //         f32::max(aabb_lt.y, aabb_lb.y),
            //         f32::max(aabb_rt.y, aabb_rb.y),
            //     );

            //     let tilemap = &renderer.tilemap;
            //     let tile_width = tilemap.tile_width;
            //     let tile_height = tilemap.tile_height;
            //     let inv_tile_width = 1f32 / tile_width;
            //     let inv_tile_height = 1f32 / tile_height;

            //     let range_min_x = min(
            //         tilemap.tile_count_x,
            //         max(0, (aabb_min_x * inv_tile_width) as isize) as usize,
            //     );
            //     let range_max_x = min(
            //         tilemap.tile_count_x,
            //         max(0, (aabb_max_x * inv_tile_width).ceil() as isize) as usize,
            //     );
            //     let range_min_y = min(
            //         tilemap.tile_count_y,
            //         max(0, (aabb_min_y * inv_tile_height) as isize) as usize,
            //     );
            //     let range_max_y = min(
            //         tilemap.tile_count_y,
            //         max(0, (aabb_max_y * inv_tile_height).ceil() as isize) as usize,
            //     );

            //     let palette = &tilemap.palette;
            //     let sprites = palette.sprites();
            //     let texture = palette.texture();
            //     let mut instance_count = 0;
            //     let mut per_instance_buffer =
            //         Vec::with_capacity(tilemap.tile_count_x * tilemap.tile_count_y * 20);

            //     for layer in &tilemap.layers {
            //         for y in range_min_y..range_max_y {
            //             let base_index = (tilemap.tile_count_y - 1 - y) * tilemap.tile_count_x;
            //             for x in range_min_x..range_max_x {
            //                 let sprite = match layer[base_index + x] {
            //                     0 => continue,
            //                     index => &sprites[index - 1],
            //                 };
            //                 let texel_mapping = sprite.texel_mapping();
            //                 let offset_x = x as f32 * tile_width;
            //                 let offset_y = y as f32 * tile_height;
            //                 let matrix = (Mat33::affine_translation(Vec2::new(offset_x, offset_y))
            //                     * matrix)
            //                     .into_elements();

            //                 instance_count += 1;
            //                 per_instance_buffer.extend([
            //                     matrix[0],
            //                     matrix[1],
            //                     matrix[2],
            //                     matrix[3],
            //                     matrix[4],
            //                     matrix[5],
            //                     matrix[6],
            //                     matrix[7],
            //                     matrix[8],
            //                     tile_width,
            //                     tile_height,
            //                     renderer.color.r,
            //                     renderer.color.g,
            //                     renderer.color.b,
            //                     renderer.color.a,
            //                     (texel_mapping.min().0 as f32 + 0.5f32) / texture.width() as f32,
            //                     (texel_mapping.min().1 as f32 + 0.5f32) / texture.height() as f32,
            //                     (texel_mapping.max().0 as f32 - 0.5f32) / texture.width() as f32,
            //                     (texel_mapping.max().1 as f32 - 0.5f32) / texture.height() as f32,
            //                 ]);
            //             }
            //         }
            //     }

            //     let shader = &renderer.shader;
            //     let mut buffer = render_mgr.alloc_buffer();

            //     buffer.replace(per_instance_buffer.as_slice());

            //     let mut r = Renderer::new(&self.renderer_bump);

            //     r.enqueue(instance_count, 2, RenderMode::Trangles, shader, |req| {
            //         render_mgr.apply_common_shader_input(shader, req);

            //         // TODO: Add shader type checking logic to alert if types have no match.

            //         if let Some(uniform) = shader.uniform("camera") {
            //             req.uniform_f33(uniform.location, world_to_ndc.elements().clone());
            //         }
            //         if let Some(uniform) = shader.uniform("sprite") {
            //             req.uniform_texture(uniform.location, &texture);
            //         }

            //         if let Some(attribute) = shader.attribute("pos") {
            //             req.attribute(
            //                 attribute.location,
            //                 &self.tilemap_sprite_buffer,
            //                 0,
            //                 attribute.ty,
            //             );
            //         }
            //         if let Some(attribute) = shader.attribute("uv") {
            //             req.attribute(
            //                 attribute.location,
            //                 &self.tilemap_sprite_buffer,
            //                 (size_of::<f32>() * 2) as _,
            //                 attribute.ty,
            //             );
            //         }

            //         if let Some(attribute) = shader.attribute("transform") {
            //             req.attribute_per_instance(attribute.location, &buffer, 0, attribute.ty);
            //         }
            //         if let Some(attribute) = shader.attribute("size") {
            //             req.attribute_per_instance(
            //                 attribute.location,
            //                 &buffer,
            //                 (size_of::<f32>() * 9) as _,
            //                 attribute.ty,
            //             );
            //         }
            //         if let Some(attribute) = shader.attribute("color") {
            //             req.attribute_per_instance(
            //                 attribute.location,
            //                 &buffer,
            //                 (size_of::<f32>() * 11) as _,
            //                 attribute.ty,
            //             );
            //         }
            //         if let Some(attribute) = shader.attribute("uv_rect") {
            //             req.attribute_per_instance(
            //                 attribute.location,
            //                 &buffer,
            //                 (size_of::<f32>() * 15) as _,
            //                 attribute.ty,
            //             );
            //         }
            //     });
            //     buffers.push(buffer);
            //     renderers.push((renderer.order, r));
            // }

            // for (transform, renderer) in (&transform, &alpha_tilemap_renderer).join() {
            //     if !Layer::has_overlap(camera.layer, renderer.layer) {
            //         return;
            //     }

            //     let matrix = transform_mgr.transform_world_matrix(transform.index());
            //     let ndc_to_local = ndc_to_world.as_ref() * matrix.inversed();

            //     let aabb_lt = (Vec3::new(-1f32, 1f32, 1f32) * ndc_to_local.as_ref()).to_vec2();
            //     let aabb_lb = (Vec3::new(-1f32, -1f32, 1f32) * ndc_to_local.as_ref()).to_vec2();
            //     let aabb_rt = (Vec3::new(1f32, 1f32, 1f32) * ndc_to_local.as_ref()).to_vec2();
            //     let aabb_rb = (Vec3::new(1f32, -1f32, 1f32) * ndc_to_local.as_ref()).to_vec2();

            //     let aabb_min_x = f32::min(
            //         f32::min(aabb_lt.x, aabb_lb.x),
            //         f32::min(aabb_rt.x, aabb_rb.x),
            //     );
            //     let aabb_max_x = f32::max(
            //         f32::max(aabb_lt.x, aabb_lb.x),
            //         f32::max(aabb_rt.x, aabb_rb.x),
            //     );
            //     let aabb_min_y = f32::min(
            //         f32::min(aabb_lt.y, aabb_lb.y),
            //         f32::min(aabb_rt.y, aabb_rb.y),
            //     );
            //     let aabb_max_y = f32::max(
            //         f32::max(aabb_lt.y, aabb_lb.y),
            //         f32::max(aabb_rt.y, aabb_rb.y),
            //     );

            //     let tilemap = &renderer.tilemap;
            //     let tile_width = tilemap.tile_width;
            //     let tile_height = tilemap.tile_height;
            //     let inv_tile_width = 1f32 / tile_width;
            //     let inv_tile_height = 1f32 / tile_height;

            //     let range_min_x = min(
            //         tilemap.tile_count_x,
            //         max(0, (aabb_min_x * inv_tile_width) as isize) as usize,
            //     );
            //     let range_max_x = min(
            //         tilemap.tile_count_x,
            //         max(0, (aabb_max_x * inv_tile_width).ceil() as isize) as usize,
            //     );
            //     let range_min_y = min(
            //         tilemap.tile_count_y,
            //         max(0, (aabb_min_y * inv_tile_height) as isize) as usize,
            //     );
            //     let range_max_y = min(
            //         tilemap.tile_count_y,
            //         max(0, (aabb_max_y * inv_tile_height).ceil() as isize) as usize,
            //     );

            //     let font = &renderer.font;
            //     let thickness = renderer.thickness;
            //     let smoothness = renderer.smoothness;
            //     let tileset = &tilemap.tileset;
            //     let mut instance_count = 0;
            //     let mut back_buffer_content =
            //         Vec::with_capacity(tilemap.tile_count_x * tilemap.tile_count_y * 19);
            //     let mut glyph_texture_and_buffers = bump_vec![in &self.extra_bump];

            //     let sdf_font_size = glyph_mgr.sdf_font_size();

            //     for y in range_min_y..range_max_y {
            //         let base_index = (tilemap.tile_count_y - 1 - y) * tilemap.tile_count_x;
            //         for x in range_min_x..range_max_x {
            //             let tile = match tilemap.layer[base_index + x] {
            //                 0 => continue,
            //                 index => &tileset.tiles[index - 1],
            //             };
            //             let tile_offset_x = x as f32 * tile_width;
            //             let tile_offset_y = y as f32 * tile_height;
            //             let back_matrix =
            //                 (Mat33::affine_translation(Vec2::new(tile_offset_x, tile_offset_y))
            //                     * matrix)
            //                     .into_elements();

            //             let glyph_key = font.lookup_glyph_index(tile.character);
            //             let g = glyph_mgr.glyph(
            //                 font,
            //                 GlyphRasterConfig {
            //                     glyph_index: glyph_key,
            //                     px: sdf_font_size,
            //                     font_hash: font.file_hash(),
            //                 },
            //             );

            //             let font_scale = renderer.font_size / sdf_font_size;
            //             let glyph_width = g.mapping.width() as f32 * font_scale;
            //             let glyph_height = g.mapping.height() as f32 * font_scale;
            //             let font_width_scale = if g.mapping.width() as usize == 2 * sdf_inset {
            //                 0f32
            //             } else {
            //                 glyph_width as f32 / (g.mapping.width() as usize - 2 * sdf_inset) as f32
            //             };
            //             let font_height_scale = if g.mapping.height() as usize == 2 * sdf_inset {
            //                 0f32
            //             } else {
            //                 glyph_height as f32
            //                     / (g.mapping.height() as usize - 2 * sdf_inset) as f32
            //             };

            //             let glyph_width =
            //                 glyph_width as f32 + 2f32 * font_width_scale * sdf_inset as f32;
            //             let glyph_height =
            //                 glyph_height as f32 + 2f32 * font_height_scale * sdf_inset as f32;
            //             let glyph_offset_x = (tilemap.tile_width - glyph_width) * 0.5f32;
            //             let glyph_offset_y = (tilemap.tile_height - glyph_height) * 0.5f32;
            //             let glyph_matrix = (Mat33::affine_translation(Vec2::new(
            //                 tile_offset_x + glyph_offset_x - sdf_inset as f32 * font_width_scale,
            //                 tile_offset_y + glyph_offset_y - sdf_inset as f32 * font_height_scale,
            //             )) * matrix)
            //                 .into_elements();

            //             instance_count += 1;
            //             back_buffer_content.extend([
            //                 back_matrix[0],
            //                 back_matrix[1],
            //                 back_matrix[2],
            //                 back_matrix[3],
            //                 back_matrix[4],
            //                 back_matrix[5],
            //                 back_matrix[6],
            //                 back_matrix[7],
            //                 back_matrix[8],
            //                 tile_width,
            //                 tile_height,
            //                 tile.back_color.r,
            //                 tile.back_color.g,
            //                 tile.back_color.b,
            //                 tile.back_color.a,
            //                 renderer.color.r,
            //                 renderer.color.g,
            //                 renderer.color.b,
            //                 renderer.color.a,
            //             ]);

            //             let mut glyph_buffer = render_mgr.alloc_buffer();
            //             glyph_buffer.replace(&[
            //                 glyph_matrix[0],
            //                 glyph_matrix[1],
            //                 glyph_matrix[2],
            //                 glyph_matrix[3],
            //                 glyph_matrix[4],
            //                 glyph_matrix[5],
            //                 glyph_matrix[6],
            //                 glyph_matrix[7],
            //                 glyph_matrix[8],
            //                 glyph_width,
            //                 glyph_height,
            //                 renderer.color.r * tile.fore_color.r,
            //                 renderer.color.g * tile.fore_color.g,
            //                 renderer.color.b * tile.fore_color.b,
            //                 renderer.color.a * tile.fore_color.a,
            //                 thickness,
            //                 smoothness,
            //                 (g.mapping.min().0 as f32) / g.texture.width() as f32,
            //                 (g.mapping.min().1 as f32) / g.texture.height() as f32,
            //                 (g.mapping.max().0 as f32) / g.texture.width() as f32,
            //                 (g.mapping.max().1 as f32) / g.texture.height() as f32,
            //             ]);
            //             glyph_texture_and_buffers.push((g.texture.handle(), glyph_buffer));
            //         }
            //     }

            //     let back_shader = &renderer.back_shader;
            //     let fore_shader = &renderer.fore_shader;
            //     let mut back_buffer = render_mgr.alloc_buffer();
            //     back_buffer.replace(back_buffer_content.as_slice());

            //     let mut r = Renderer::new(&self.renderer_bump);

            //     r.enqueue(
            //         instance_count,
            //         2,
            //         RenderMode::Trangles,
            //         back_shader,
            //         |req| {
            //             render_mgr.apply_common_shader_input(back_shader, req);

            //             // TODO: Add shader type checking logic to alert if types have no match.

            //             if let Some(uniform) = back_shader.uniform("camera") {
            //                 req.uniform_f33(uniform.location, world_to_ndc.elements().clone());
            //             }

            //             if let Some(attribute) = back_shader.attribute("pos") {
            //                 req.attribute(
            //                     attribute.location,
            //                     &self.alpha_tilemap_back_buffer,
            //                     0,
            //                     attribute.ty,
            //                 );
            //             }

            //             if let Some(attribute) = back_shader.attribute("transform") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &back_buffer,
            //                     0,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = back_shader.attribute("size") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &back_buffer,
            //                     (size_of::<f32>() * 9) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = back_shader.attribute("color") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &back_buffer,
            //                     (size_of::<f32>() * 11) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = back_shader.attribute("tint_color") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &back_buffer,
            //                     (size_of::<f32>() * 15) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //         },
            //     );
            //     buffers.push(back_buffer);

            //     for (texture, buffer) in glyph_texture_and_buffers {
            //         r.enqueue(1, 2, RenderMode::Trangles, fore_shader, |req| {
            //             render_mgr.apply_common_shader_input(fore_shader, req);

            //             // TODO: Add shader type checking logic to alert if types have no match.

            //             if let Some(uniform) = fore_shader.uniform("camera") {
            //                 req.uniform_f33(uniform.location, world_to_ndc.elements().clone());
            //             }
            //             if let Some(uniform) = fore_shader.uniform("glyph") {
            //                 req.uniform_texture_raw(uniform.location, texture);
            //             }

            //             if let Some(attribute) = fore_shader.attribute("pos") {
            //                 req.attribute(attribute.location, &self.glyph_buffer, 0, attribute.ty);
            //             }
            //             if let Some(attribute) = fore_shader.attribute("uv") {
            //                 req.attribute(
            //                     attribute.location,
            //                     &self.glyph_buffer,
            //                     (size_of::<f32>() * 2) as _,
            //                     attribute.ty,
            //                 );
            //             }

            //             if let Some(attribute) = fore_shader.attribute("transform") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &buffer,
            //                     0,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = fore_shader.attribute("size") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &buffer,
            //                     (size_of::<f32>() * 9) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = fore_shader.attribute("color") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &buffer,
            //                     (size_of::<f32>() * 11) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = fore_shader.attribute("thickness") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &buffer,
            //                     (size_of::<f32>() * 15) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = fore_shader.attribute("smoothness") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &buffer,
            //                     (size_of::<f32>() * 16) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //             if let Some(attribute) = fore_shader.attribute("uv_rect") {
            //                 req.attribute_per_instance(
            //                     attribute.location,
            //                     &buffer,
            //                     (size_of::<f32>() * 17) as _,
            //                     attribute.ty,
            //                 );
            //             }
            //         });
            //         buffers.push(buffer);
            //     }
            //     renderers.push((renderer.order, r));
            // }

            // renderers.sort_unstable_by_key(|(order, _)| *order);

            // for (_, renderer) in renderers {
            //     renderer.flush();
            // }

            // for buffer in buffers {
            //     render_mgr.dealloc_buffer(buffer);
            // }
        }

        let encoders = [
            render_mgr.submit_frame_memory_allocation().finish(),
            encoder.finish(),
        ];
        render_mgr.queue().submit(encoders);
        surface_texture.present();
    }
}

struct CameraWithTransformBuffer<'c> {
    pub camera: &'c Camera,
    pub transform: &'c Transform,
    pub transform_buffer: DeviceAllocation,
}

struct RenderRequest<'r> {
    pub pipeline: PipelineHandle,
    pub bind_group: &'r BindGroup,
    pub per_vertex_buffer: &'r Buffer,
    pub per_instance_buffer: DeviceAllocation,
    pub per_instance_data: HostAllocation,
}

#[derive(Debug, Clone, Copy)]
struct RenderRequestIndex {
    pub order: i32,
    pub hash: u64, // Hash of pipeline + bind_group + per_instance_buffer; for faster computation.
    pub buffer_begin: u32,
    pub buffer_end: u32,
    pub request_index: u32,
}

impl RenderRequestIndex {
    pub fn from_request(request_index: u32, order: i32, request: &RenderRequest) -> Self {
        let range = request.per_instance_buffer.range();
        debug_assert!(range.start <= u32::MAX as u64);
        debug_assert!(range.end <= u32::MAX as u64);
        Self {
            order,
            hash: {
                let mut hasher = DefaultHasher::new();
                request.pipeline.as_ptr().hash(&mut hasher);
                (request.bind_group as *const BindGroup).hash(&mut hasher);
                Arc::as_ptr(request.per_instance_buffer.buffer()).hash(&mut hasher);
                hasher.finish()
            },
            buffer_begin: range.start as u32,
            buffer_end: range.end as u32,
            request_index,
        }
    }

    pub fn can_be_merged(&self, before: &Self) -> bool {
        self.hash == before.hash && self.buffer_begin == before.buffer_end
    }
}

impl PartialEq for RenderRequestIndex {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash && self.buffer_begin == other.buffer_begin
    }
}

impl Eq for RenderRequestIndex {}

impl PartialOrd for RenderRequestIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RenderRequestIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.order.cmp(&other.order) {
            Ordering::Equal => {}
            ordering @ _ => return ordering,
        }

        match self.hash.cmp(&other.hash) {
            Ordering::Equal => {}
            ordering @ _ => return ordering,
        }

        match self.buffer_begin.cmp(&other.buffer_begin) {
            Ordering::Equal => {}
            ordering @ _ => return ordering,
        }

        Ordering::Equal
    }
}
