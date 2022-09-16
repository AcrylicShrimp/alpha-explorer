use crate::component::*;
use crate::engine::use_context;
use crate::render::*;
use crate::structure::Vec2;
use bumpalo::vec as bump_vec;
use bumpalo::Bump;
use fontdue::layout::GlyphRasterConfig;
use fontdue::layout::HorizontalAlign;
use fontdue::layout::VerticalAlign;
use specs::prelude::*;
use std::cmp::{max, min};
use std::iter::FromIterator;
use std::mem::size_of;

pub struct RenderSystem {
    renderer_bump: Bump,
    extra_bump: Bump,
    glyph_buffer: Buffer,
    sprite_buffer: Buffer,
    tilemap_sprite_buffer: Buffer,
    alpha_tilemap_back_buffer: Buffer,
}

impl RenderSystem {
    pub fn new() -> Self {
        let glyph_buffer = Buffer::from_slice(&[
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
        let sprite_buffer = Buffer::from_slice(&[
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
        let tilemap_sprite_buffer = Buffer::from_slice(&[
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
        let alpha_tilemap_back_buffer = Buffer::from_slice(&[
            1f32, 1f32, //
            //
            0f32, 0f32, //
            //
            1f32, 0f32, //
            //
            //
            //
            1f32, 1f32, //
            //
            0f32, 1f32, //
            //
            0f32, 0f32, //
        ]);

        Self {
            renderer_bump: Bump::with_capacity(4 * 1024),
            extra_bump: Bump::with_capacity(1 * 1024),
            glyph_buffer,
            sprite_buffer,
            tilemap_sprite_buffer,
            alpha_tilemap_back_buffer,
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Camera>,
        WriteStorage<'a, GlyphRenderer>,
        ReadStorage<'a, SpriteRenderer>,
        ReadStorage<'a, NinePatchRenderer>,
        ReadStorage<'a, TilemapRenderer>,
        ReadStorage<'a, AlphaTilemapRenderer>,
    );

    fn run(
        &mut self,
        (
            transform,
            size,
            camera,
            mut glyph_renderer,
            sprite_renderer,
            nine_patch_renderer,
            tilemap_renderer,
            alpha_tilemap_renderer,
        ): Self::SystemData,
    ) {
        self.renderer_bump.reset();
        self.extra_bump.reset();

        let context = use_context();
        let mut render_mgr = context.render_mgr_mut();
        let screen_mgr = context.screen_mgr();
        let transform_mgr = context.transform_mgr();
        let mut glyph_mgr = context.glyph_mgr_mut();

        let cameras = (&transform, &camera).join();
        let mut cameras = Vec::from_iter(cameras);
        cameras.sort_unstable_by(|lhs, rhs| lhs.1.order.cmp(&rhs.1.order));

        let width_half = (screen_mgr.width() * 0.5) as f32;
        let height_half = (screen_mgr.height() * 0.5) as f32;

        for (camera_transform, camera) in cameras {
            let camera_transform_index = camera_transform.index();
            let mut ndc_to_world = transform_mgr
                .transform_world_matrix(camera_transform_index)
                .clone();

            let mut camera_matrix_inverse = [0f32; 9];
            camera_matrix_inverse.clone_from(&ndc_to_world);
            crate::transform::Transform::inverse_matrix_with_scale(
                width_half,
                height_half,
                &mut camera_matrix_inverse,
            );

            ndc_to_world[0] *= width_half;
            ndc_to_world[1] *= width_half;
            ndc_to_world[3] *= height_half;
            ndc_to_world[4] *= height_half;

            let mut buffers = bump_vec![in &self.extra_bump];
            let mut renderers = bump_vec![in &self.extra_bump];
            let sdf_inset = glyph_mgr.sdf_inset();

            for (transform, size, renderer) in (&transform, &size, &mut glyph_renderer).join() {
                if !Layer::has_overlap(camera.layer, renderer.layer) {
                    return;
                }

                let size = size.size;
                let color = renderer.color;
                let thickness = renderer.thickness;
                let smoothness = renderer.smoothness;
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
                let matrix = transform_mgr.transform_world_matrix(transform.index());
                let mut texture_and_buffers = bump_vec![in &self.extra_bump];

                let (font, layout) = renderer.font_and_layout();

                for glyph in layout.glyphs() {
                    let g = glyph_mgr.glyph(font, glyph.key);
                    let mut buffer = render_mgr.alloc_buffer();

                    let font_width_scale = if g.mapping.width() as usize == 2 * sdf_inset {
                        0f32
                    } else {
                        glyph.width as f32 / (g.mapping.width() as usize - 2 * sdf_inset) as f32
                    };
                    let font_height_scale = if g.mapping.height() as usize == 2 * sdf_inset {
                        0f32
                    } else {
                        glyph.height as f32 / (g.mapping.height() as usize - 2 * sdf_inset) as f32
                    };

                    let glyph_width =
                        glyph.width as f32 + 2f32 * font_width_scale * sdf_inset as f32;
                    let glyph_height =
                        glyph.height as f32 + 2f32 * font_height_scale * sdf_inset as f32;

                    buffer.replace(&[
                        matrix[0],
                        matrix[1],
                        matrix[2],
                        matrix[3],
                        matrix[4],
                        matrix[5],
                        matrix[6]
                            + matrix[0]
                                * (glyph.x + offset.x - sdf_inset as f32 * font_width_scale)
                            + matrix[3]
                                * (glyph.y - offset.y - sdf_inset as f32 * font_height_scale),
                        matrix[7]
                            + matrix[1]
                                * (glyph.x + offset.x - sdf_inset as f32 * font_width_scale)
                            + matrix[4]
                                * (glyph.y - offset.y - sdf_inset as f32 * font_height_scale),
                        matrix[8],
                        glyph_width,
                        glyph_height,
                        color.r,
                        color.g,
                        color.b,
                        color.a,
                        thickness,
                        smoothness,
                        (g.mapping.min().0 as f32) / g.texture.width() as f32,
                        (g.mapping.min().1 as f32) / g.texture.height() as f32,
                        (g.mapping.max().0 as f32) / g.texture.width() as f32,
                        (g.mapping.max().1 as f32) / g.texture.height() as f32,
                    ]);

                    texture_and_buffers.push((g.texture.handle(), buffer));
                }

                let shader = &renderer.shader;
                let mut r = Renderer::new(&self.renderer_bump);

                // TODO: Merge instances that have the same texture to reduce draw calls.

                for (texture, buffer) in texture_and_buffers {
                    r.enqueue(1, 2, RenderMode::Trangles, shader, |req| {
                        render_mgr.apply_common_shader_input(shader, req);

                        // TODO: Add shader type checking logic to alert if types have no match.

                        if let Some(uniform) = shader.uniform("camera") {
                            req.uniform_f33(uniform.location, camera_matrix_inverse);
                        }
                        if let Some(uniform) = shader.uniform("glyph") {
                            req.uniform_texture_raw(uniform.location, texture);
                        }

                        if let Some(attribute) = shader.attribute("pos") {
                            req.attribute(attribute.location, &self.glyph_buffer, 0, attribute.ty);
                        }
                        if let Some(attribute) = shader.attribute("uv") {
                            req.attribute(
                                attribute.location,
                                &self.glyph_buffer,
                                (size_of::<f32>() * 2) as _,
                                attribute.ty,
                            );
                        }

                        if let Some(attribute) = shader.attribute("transform") {
                            req.attribute_per_instance(
                                attribute.location,
                                &buffer,
                                0,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = shader.attribute("size") {
                            req.attribute_per_instance(
                                attribute.location,
                                &buffer,
                                (size_of::<f32>() * 9) as _,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = shader.attribute("color") {
                            req.attribute_per_instance(
                                attribute.location,
                                &buffer,
                                (size_of::<f32>() * 11) as _,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = shader.attribute("thickness") {
                            req.attribute_per_instance(
                                attribute.location,
                                &buffer,
                                (size_of::<f32>() * 15) as _,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = shader.attribute("smoothness") {
                            req.attribute_per_instance(
                                attribute.location,
                                &buffer,
                                (size_of::<f32>() * 16) as _,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = shader.attribute("uv_rect") {
                            req.attribute_per_instance(
                                attribute.location,
                                &buffer,
                                (size_of::<f32>() * 17) as _,
                                attribute.ty,
                            );
                        }
                    });
                    buffers.push(buffer);
                }

                renderers.push((renderer.order, r));
            }

            for (transform, size, renderer) in (&transform, &size, &sprite_renderer).join() {
                if !Layer::has_overlap(camera.layer, renderer.layer) {
                    return;
                }

                let size = size.size;
                let matrix = transform_mgr.transform_world_matrix(transform.index());
                let sprite = &renderer.sprite;
                let mut buffer = render_mgr.alloc_buffer();

                buffer.replace(&[
                    matrix[0],
                    matrix[1],
                    matrix[2],
                    matrix[3],
                    matrix[4],
                    matrix[5],
                    matrix[6]
                        + matrix[0] * (-size.width * 0.5f32)
                        + matrix[3] * (size.height * 0.5f32),
                    matrix[7]
                        + matrix[1] * (-size.width * 0.5f32)
                        + matrix[4] * (size.height * 0.5f32),
                    matrix[8],
                    size.width,
                    size.height,
                    renderer.color.r,
                    renderer.color.g,
                    renderer.color.b,
                    renderer.color.a,
                    (sprite.texel_mapping().min().0 as f32) / sprite.texture().width() as f32,
                    (sprite.texel_mapping().min().1 as f32) / sprite.texture().height() as f32,
                    (sprite.texel_mapping().max().0 as f32) / sprite.texture().width() as f32,
                    (sprite.texel_mapping().max().1 as f32) / sprite.texture().height() as f32,
                ]);

                let shader = &renderer.shader;
                let mut r = Renderer::new(&self.renderer_bump);

                r.enqueue(1, 2, RenderMode::Trangles, shader, |req| {
                    render_mgr.apply_common_shader_input(shader, req);

                    // TODO: Add shader type checking logic to alert if types have no match.

                    if let Some(uniform) = shader.uniform("camera") {
                        req.uniform_f33(uniform.location, camera_matrix_inverse);
                    }
                    if let Some(uniform) = shader.uniform("sprite") {
                        req.uniform_texture(uniform.location, sprite.texture());
                    }

                    if let Some(attribute) = shader.attribute("pos") {
                        req.attribute(attribute.location, &self.sprite_buffer, 0, attribute.ty);
                    }
                    if let Some(attribute) = shader.attribute("uv") {
                        req.attribute(
                            attribute.location,
                            &self.sprite_buffer,
                            (size_of::<f32>() * 2) as _,
                            attribute.ty,
                        );
                    }

                    if let Some(attribute) = shader.attribute("transform") {
                        req.attribute_per_instance(attribute.location, &buffer, 0, attribute.ty);
                    }
                    if let Some(attribute) = shader.attribute("size") {
                        req.attribute_per_instance(
                            attribute.location,
                            &buffer,
                            (size_of::<f32>() * 9) as _,
                            attribute.ty,
                        );
                    }
                    if let Some(attribute) = shader.attribute("color") {
                        req.attribute_per_instance(
                            attribute.location,
                            &buffer,
                            (size_of::<f32>() * 11) as _,
                            attribute.ty,
                        );
                    }
                    if let Some(attribute) = shader.attribute("uv_rect") {
                        req.attribute_per_instance(
                            attribute.location,
                            &buffer,
                            (size_of::<f32>() * 15) as _,
                            attribute.ty,
                        );
                    }
                });
                buffers.push(buffer);
                renderers.push((renderer.order, r));
            }

            for (transform, size, renderer) in (&transform, &size, &nine_patch_renderer).join() {
                if !Layer::has_overlap(camera.layer, renderer.layer) {
                    return;
                }

                let size = size.size;
                let matrix = transform_mgr.transform_world_matrix(transform.index());
                let nine_patch = &renderer.nine_patch;

                let left = nine_patch.sprite_lt().width() as f32;
                let right = nine_patch.sprite_rt().width() as f32;
                let center = f32::max(0f32, size.width - left - right);
                let (left, right) = if 0f32 < center {
                    (left, right)
                } else {
                    let ratio = size.width / (left + right);
                    (left * ratio, right * ratio)
                };

                let top = nine_patch.sprite_lt().height() as f32;
                let bottom = nine_patch.sprite_lb().height() as f32;
                let middle = f32::max(0f32, size.height - top - bottom);
                let (top, bottom) = if 0f32 < middle {
                    (top, bottom)
                } else {
                    let ratio = size.height / (top + bottom);
                    (top * ratio, bottom * ratio)
                };

                let mut buffer_data: Vec<f32> = Vec::with_capacity(9 * 19);
                let mut enqueue_patch = |offset_x: f32,
                                         offset_y: f32,
                                         width: f32,
                                         height: f32,
                                         sprite: &Sprite| {
                    buffer_data.extend(&[
                        matrix[0],
                        matrix[1],
                        matrix[2],
                        matrix[3],
                        matrix[4],
                        matrix[5],
                        matrix[6]
                            + matrix[0] * (offset_x - size.width * 0.5f32)
                            + matrix[3] * (offset_y + size.height * 0.5f32),
                        matrix[7]
                            + matrix[1] * (offset_x - size.width * 0.5f32)
                            + matrix[4] * (offset_y + size.height * 0.5f32),
                        matrix[8],
                        width,
                        height,
                        renderer.color.r,
                        renderer.color.g,
                        renderer.color.b,
                        renderer.color.a,
                        (sprite.texel_mapping().min().0 as f32) / sprite.texture().width() as f32,
                        (sprite.texel_mapping().min().1 as f32) / sprite.texture().height() as f32,
                        (sprite.texel_mapping().max().0 as f32) / sprite.texture().width() as f32,
                        (sprite.texel_mapping().max().1 as f32) / sprite.texture().height() as f32,
                    ]);
                };

                let mut patch_count = 0;

                if 0f32 < left && 0f32 < top {
                    patch_count += 1;
                    enqueue_patch(0f32, -top, left, top, &nine_patch.sprite_lt());
                }

                if 0f32 < center && 0f32 < top {
                    patch_count += 1;
                    enqueue_patch(left, -top, center, top, &nine_patch.sprite_ct());
                }

                if 0f32 < right && 0f32 < top {
                    patch_count += 1;
                    enqueue_patch(left + center, -top, right, top, &nine_patch.sprite_rt());
                }

                if 0f32 < left && 0f32 < middle {
                    patch_count += 1;
                    enqueue_patch(0f32, -(top + middle), left, middle, &nine_patch.sprite_lm());
                }

                if 0f32 < center && 0f32 < middle {
                    patch_count += 1;
                    enqueue_patch(
                        left,
                        -(top + middle),
                        center,
                        middle,
                        &nine_patch.sprite_cm(),
                    );
                }

                if 0f32 < right && 0f32 < middle {
                    patch_count += 1;
                    enqueue_patch(
                        left + center,
                        -(top + middle),
                        right,
                        middle,
                        &nine_patch.sprite_rm(),
                    );
                }

                if 0f32 < left && 0f32 < bottom {
                    patch_count += 1;
                    enqueue_patch(
                        0f32,
                        -(top + middle + bottom),
                        left,
                        bottom,
                        &nine_patch.sprite_lb(),
                    );
                }

                if 0f32 < center && 0f32 < bottom {
                    patch_count += 1;
                    enqueue_patch(
                        left,
                        -(top + middle + bottom),
                        center,
                        bottom,
                        &nine_patch.sprite_cb(),
                    );
                }

                if 0f32 < right && 0f32 < bottom {
                    patch_count += 1;
                    enqueue_patch(
                        left + center,
                        -(top + middle + bottom),
                        right,
                        bottom,
                        &nine_patch.sprite_rb(),
                    );
                }

                let mut buffer = render_mgr.alloc_buffer();
                buffer.replace(buffer_data.as_slice());

                let shader = &renderer.shader;
                let mut r = Renderer::new(&self.renderer_bump);

                r.enqueue(patch_count, 2, RenderMode::Trangles, shader, |req| {
                    render_mgr.apply_common_shader_input(shader, req);

                    // TODO: Add shader type checking logic to alert if types have no match.

                    if let Some(uniform) = shader.uniform("camera") {
                        req.uniform_f33(uniform.location, camera_matrix_inverse);
                    }
                    if let Some(uniform) = shader.uniform("sprite") {
                        req.uniform_texture(uniform.location, nine_patch.texture());
                    }

                    if let Some(attribute) = shader.attribute("pos") {
                        req.attribute(attribute.location, &self.sprite_buffer, 0, attribute.ty);
                    }
                    if let Some(attribute) = shader.attribute("uv") {
                        req.attribute(
                            attribute.location,
                            &self.sprite_buffer,
                            (size_of::<f32>() * 2) as _,
                            attribute.ty,
                        );
                    }

                    if let Some(attribute) = shader.attribute("transform") {
                        req.attribute_per_instance(attribute.location, &buffer, 0, attribute.ty);
                    }
                    if let Some(attribute) = shader.attribute("size") {
                        req.attribute_per_instance(
                            attribute.location,
                            &buffer,
                            (size_of::<f32>() * 9) as _,
                            attribute.ty,
                        );
                    }
                    if let Some(attribute) = shader.attribute("color") {
                        req.attribute_per_instance(
                            attribute.location,
                            &buffer,
                            (size_of::<f32>() * 11) as _,
                            attribute.ty,
                        );
                    }
                    if let Some(attribute) = shader.attribute("uv_rect") {
                        req.attribute_per_instance(
                            attribute.location,
                            &buffer,
                            (size_of::<f32>() * 15) as _,
                            attribute.ty,
                        );
                    }
                });
                buffers.push(buffer);
                renderers.push((renderer.order, r));
            }

            for (transform, renderer) in (&transform, &tilemap_renderer).join() {
                if !Layer::has_overlap(camera.layer, renderer.layer) {
                    return;
                }

                let matrix = transform_mgr.transform_world_matrix(transform.index());
                let mut world_to_local = [0f32; 9];
                let mut ndc_to_local = [0f32; 6];

                world_to_local.clone_from(&matrix);
                crate::transform::Transform::inverse_matrix(&mut world_to_local);

                ndc_to_local[0] = ndc_to_world[0] * world_to_local[0]
                    + ndc_to_world[1] * world_to_local[3]
                    + ndc_to_world[2] * world_to_local[6];
                ndc_to_local[1] = ndc_to_world[0] * world_to_local[1]
                    + ndc_to_world[1] * world_to_local[4]
                    + ndc_to_world[2] * world_to_local[7];
                ndc_to_local[2] = ndc_to_world[3] * world_to_local[0]
                    + ndc_to_world[4] * world_to_local[3]
                    + ndc_to_world[5] * world_to_local[6];
                ndc_to_local[3] = ndc_to_world[3] * world_to_local[1]
                    + ndc_to_world[4] * world_to_local[4]
                    + ndc_to_world[5] * world_to_local[7];
                ndc_to_local[4] = ndc_to_world[6] * world_to_local[0]
                    + ndc_to_world[7] * world_to_local[3]
                    + ndc_to_world[8] * world_to_local[6];
                ndc_to_local[5] = ndc_to_world[6] * world_to_local[1]
                    + ndc_to_world[7] * world_to_local[4]
                    + ndc_to_world[8] * world_to_local[7];

                let aabb_x_lt = -ndc_to_local[0] + ndc_to_local[2] + ndc_to_local[4];
                let aabb_x_lb = -ndc_to_local[0] - ndc_to_local[2] + ndc_to_local[4];
                let aabb_x_rt = ndc_to_local[0] + ndc_to_local[2] + ndc_to_local[4];
                let aabb_x_rb = ndc_to_local[0] - ndc_to_local[2] + ndc_to_local[4];

                let aabb_y_lt = -ndc_to_local[1] + ndc_to_local[3] + ndc_to_local[5];
                let aabb_y_lb = -ndc_to_local[1] + -ndc_to_local[3] + ndc_to_local[5];
                let aabb_y_rt = ndc_to_local[1] + ndc_to_local[3] + ndc_to_local[5];
                let aabb_y_rb = ndc_to_local[1] + -ndc_to_local[3] + ndc_to_local[5];

                let aabb_min_x = [aabb_x_lt, aabb_x_lb, aabb_x_rt, aabb_x_rb]
                    .iter()
                    .cloned()
                    .reduce(f32::min)
                    .unwrap();
                let aabb_max_x = [aabb_x_lt, aabb_x_lb, aabb_x_rt, aabb_x_rb]
                    .iter()
                    .cloned()
                    .reduce(f32::max)
                    .unwrap();
                let aabb_min_y = [aabb_y_lt, aabb_y_lb, aabb_y_rt, aabb_y_rb]
                    .iter()
                    .cloned()
                    .reduce(f32::min)
                    .unwrap();
                let aabb_max_y = [aabb_y_lt, aabb_y_lb, aabb_y_rt, aabb_y_rb]
                    .iter()
                    .cloned()
                    .reduce(f32::max)
                    .unwrap();

                let tilemap = &renderer.tilemap;
                let tile_width = tilemap.tile_width;
                let tile_height = tilemap.tile_height;
                let inv_tile_width = 1f32 / tile_width;
                let inv_tile_height = 1f32 / tile_height;

                let range_min_x = min(
                    tilemap.tile_count_x,
                    max(0, (aabb_min_x * inv_tile_width) as isize) as usize,
                );
                let range_max_x = min(
                    tilemap.tile_count_x,
                    max(0, (aabb_max_x * inv_tile_width).ceil() as isize) as usize,
                );
                let range_min_y = min(
                    tilemap.tile_count_y,
                    max(0, (aabb_min_y * inv_tile_height) as isize) as usize,
                );
                let range_max_y = min(
                    tilemap.tile_count_y,
                    max(0, (aabb_max_y * inv_tile_height).ceil() as isize) as usize,
                );

                let palette = &tilemap.palette;
                let sprites = palette.sprites();
                let texture = palette.texture();
                let mut instance_count = 0;
                let mut per_instance_buffer =
                    Vec::with_capacity(tilemap.tile_count_x * tilemap.tile_count_y * 20);

                for layer in &tilemap.layers {
                    for y in range_min_y..range_max_y {
                        let base_index = (tilemap.tile_count_y - 1 - y) * tilemap.tile_count_x;
                        for x in range_min_x..range_max_x {
                            let sprite = match layer[base_index + x] {
                                0 => continue,
                                index => &sprites[index - 1],
                            };
                            let texel_mapping = sprite.texel_mapping();
                            let offset_x = x as f32 * tile_width;
                            let offset_y = y as f32 * tile_height;

                            instance_count += 1;
                            per_instance_buffer.extend([
                                matrix[0],
                                matrix[1],
                                matrix[2],
                                matrix[3],
                                matrix[4],
                                matrix[5],
                                matrix[6] + matrix[0] * offset_x + matrix[3] * offset_y,
                                matrix[7] + matrix[1] * offset_x + matrix[4] * offset_y,
                                matrix[8],
                                tile_width,
                                tile_height,
                                renderer.color.r,
                                renderer.color.g,
                                renderer.color.b,
                                renderer.color.a,
                                (texel_mapping.min().0 as f32 + 0.5f32) / texture.width() as f32,
                                (texel_mapping.min().1 as f32 + 0.5f32) / texture.height() as f32,
                                (texel_mapping.max().0 as f32 - 0.5f32) / texture.width() as f32,
                                (texel_mapping.max().1 as f32 - 0.5f32) / texture.height() as f32,
                            ]);
                        }
                    }
                }

                let shader = &renderer.shader;
                let mut buffer = render_mgr.alloc_buffer();

                buffer.replace(per_instance_buffer.as_slice());

                let mut r = Renderer::new(&self.renderer_bump);

                r.enqueue(instance_count, 2, RenderMode::Trangles, shader, |req| {
                    render_mgr.apply_common_shader_input(shader, req);

                    // TODO: Add shader type checking logic to alert if types have no match.

                    if let Some(uniform) = shader.uniform("camera") {
                        req.uniform_f33(uniform.location, camera_matrix_inverse);
                    }
                    if let Some(uniform) = shader.uniform("sprite") {
                        req.uniform_texture(uniform.location, &texture);
                    }

                    if let Some(attribute) = shader.attribute("pos") {
                        req.attribute(
                            attribute.location,
                            &self.tilemap_sprite_buffer,
                            0,
                            attribute.ty,
                        );
                    }
                    if let Some(attribute) = shader.attribute("uv") {
                        req.attribute(
                            attribute.location,
                            &self.tilemap_sprite_buffer,
                            (size_of::<f32>() * 2) as _,
                            attribute.ty,
                        );
                    }

                    if let Some(attribute) = shader.attribute("transform") {
                        req.attribute_per_instance(attribute.location, &buffer, 0, attribute.ty);
                    }
                    if let Some(attribute) = shader.attribute("size") {
                        req.attribute_per_instance(
                            attribute.location,
                            &buffer,
                            (size_of::<f32>() * 9) as _,
                            attribute.ty,
                        );
                    }
                    if let Some(attribute) = shader.attribute("color") {
                        req.attribute_per_instance(
                            attribute.location,
                            &buffer,
                            (size_of::<f32>() * 11) as _,
                            attribute.ty,
                        );
                    }
                    if let Some(attribute) = shader.attribute("uv_rect") {
                        req.attribute_per_instance(
                            attribute.location,
                            &buffer,
                            (size_of::<f32>() * 15) as _,
                            attribute.ty,
                        );
                    }
                });
                buffers.push(buffer);
                renderers.push((renderer.order, r));
            }

            for (transform, renderer) in (&transform, &alpha_tilemap_renderer).join() {
                if !Layer::has_overlap(camera.layer, renderer.layer) {
                    return;
                }

                let matrix = transform_mgr.transform_world_matrix(transform.index());
                let mut world_to_local = [0f32; 9];
                let mut ndc_to_local = [0f32; 6];

                world_to_local.clone_from(&matrix);
                crate::transform::Transform::inverse_matrix(&mut world_to_local);

                ndc_to_local[0] = ndc_to_world[0] * world_to_local[0]
                    + ndc_to_world[1] * world_to_local[3]
                    + ndc_to_world[2] * world_to_local[6];
                ndc_to_local[1] = ndc_to_world[0] * world_to_local[1]
                    + ndc_to_world[1] * world_to_local[4]
                    + ndc_to_world[2] * world_to_local[7];
                ndc_to_local[2] = ndc_to_world[3] * world_to_local[0]
                    + ndc_to_world[4] * world_to_local[3]
                    + ndc_to_world[5] * world_to_local[6];
                ndc_to_local[3] = ndc_to_world[3] * world_to_local[1]
                    + ndc_to_world[4] * world_to_local[4]
                    + ndc_to_world[5] * world_to_local[7];
                ndc_to_local[4] = ndc_to_world[6] * world_to_local[0]
                    + ndc_to_world[7] * world_to_local[3]
                    + ndc_to_world[8] * world_to_local[6];
                ndc_to_local[5] = ndc_to_world[6] * world_to_local[1]
                    + ndc_to_world[7] * world_to_local[4]
                    + ndc_to_world[8] * world_to_local[7];

                let aabb_x_lt = -ndc_to_local[0] + ndc_to_local[2] + ndc_to_local[4];
                let aabb_x_lb = -ndc_to_local[0] - ndc_to_local[2] + ndc_to_local[4];
                let aabb_x_rt = ndc_to_local[0] + ndc_to_local[2] + ndc_to_local[4];
                let aabb_x_rb = ndc_to_local[0] - ndc_to_local[2] + ndc_to_local[4];

                let aabb_y_lt = -ndc_to_local[1] + ndc_to_local[3] + ndc_to_local[5];
                let aabb_y_lb = -ndc_to_local[1] + -ndc_to_local[3] + ndc_to_local[5];
                let aabb_y_rt = ndc_to_local[1] + ndc_to_local[3] + ndc_to_local[5];
                let aabb_y_rb = ndc_to_local[1] + -ndc_to_local[3] + ndc_to_local[5];

                let aabb_min_x = [aabb_x_lt, aabb_x_lb, aabb_x_rt, aabb_x_rb]
                    .iter()
                    .cloned()
                    .reduce(f32::min)
                    .unwrap();
                let aabb_max_x = [aabb_x_lt, aabb_x_lb, aabb_x_rt, aabb_x_rb]
                    .iter()
                    .cloned()
                    .reduce(f32::max)
                    .unwrap();
                let aabb_min_y = [aabb_y_lt, aabb_y_lb, aabb_y_rt, aabb_y_rb]
                    .iter()
                    .cloned()
                    .reduce(f32::min)
                    .unwrap();
                let aabb_max_y = [aabb_y_lt, aabb_y_lb, aabb_y_rt, aabb_y_rb]
                    .iter()
                    .cloned()
                    .reduce(f32::max)
                    .unwrap();

                let tilemap = &renderer.tilemap;
                let tile_width = tilemap.tile_width;
                let tile_height = tilemap.tile_height;
                let inv_tile_width = 1f32 / tile_width;
                let inv_tile_height = 1f32 / tile_height;

                let range_min_x = min(
                    tilemap.tile_count_x,
                    max(0, (aabb_min_x * inv_tile_width) as isize) as usize,
                );
                let range_max_x = min(
                    tilemap.tile_count_x,
                    max(0, (aabb_max_x * inv_tile_width).ceil() as isize) as usize,
                );
                let range_min_y = min(
                    tilemap.tile_count_y,
                    max(0, (aabb_min_y * inv_tile_height) as isize) as usize,
                );
                let range_max_y = min(
                    tilemap.tile_count_y,
                    max(0, (aabb_max_y * inv_tile_height).ceil() as isize) as usize,
                );

                let font = &renderer.font;
                let thickness = renderer.thickness;
                let smoothness = renderer.smoothness;
                let tileset = &tilemap.tileset;
                let mut instance_count = 0;
                let mut back_buffer_content =
                    Vec::with_capacity(tilemap.tile_count_x * tilemap.tile_count_y * 19);
                let mut glyph_texture_and_buffers = bump_vec![in &self.extra_bump];

                let sdf_font_size = glyph_mgr.sdf_font_size();

                for y in range_min_y..range_max_y {
                    let base_index = (tilemap.tile_count_y - 1 - y) * tilemap.tile_count_x;
                    for x in range_min_x..range_max_x {
                        let tile = match tilemap.layer[base_index + x] {
                            0 => continue,
                            index => &tileset.tiles[index - 1],
                        };
                        let tile_offset_x = x as f32 * tile_width;
                        let tile_offset_y = y as f32 * tile_height;

                        let glyph_key = font.lookup_glyph_index(tile.character);
                        let g = glyph_mgr.glyph(
                            font,
                            GlyphRasterConfig {
                                glyph_index: glyph_key,
                                px: sdf_font_size,
                                font_hash: font.file_hash(),
                            },
                        );

                        let font_scale = renderer.font_size / sdf_font_size;
                        let glyph_width = g.mapping.width() as f32 * font_scale;
                        let glyph_height = g.mapping.height() as f32 * font_scale;
                        let font_width_scale = if g.mapping.width() as usize == 2 * sdf_inset {
                            0f32
                        } else {
                            glyph_width as f32 / (g.mapping.width() as usize - 2 * sdf_inset) as f32
                        };
                        let font_height_scale = if g.mapping.height() as usize == 2 * sdf_inset {
                            0f32
                        } else {
                            glyph_height as f32
                                / (g.mapping.height() as usize - 2 * sdf_inset) as f32
                        };

                        let glyph_width =
                            glyph_width as f32 + 2f32 * font_width_scale * sdf_inset as f32;
                        let glyph_height =
                            glyph_height as f32 + 2f32 * font_height_scale * sdf_inset as f32;
                        let glyph_offset_x = (tilemap.tile_width - glyph_width) * 0.5f32;
                        let glyph_offset_y = (tilemap.tile_height - glyph_height) * 0.5f32;

                        instance_count += 1;
                        back_buffer_content.extend([
                            matrix[0],
                            matrix[1],
                            matrix[2],
                            matrix[3],
                            matrix[4],
                            matrix[5],
                            matrix[6] + matrix[0] * tile_offset_x + matrix[3] * tile_offset_y,
                            matrix[7] + matrix[1] * tile_offset_x + matrix[4] * tile_offset_y,
                            matrix[8],
                            tile_width,
                            tile_height,
                            tile.back_color.r,
                            tile.back_color.g,
                            tile.back_color.b,
                            tile.back_color.a,
                            renderer.color.r,
                            renderer.color.g,
                            renderer.color.b,
                            renderer.color.a,
                        ]);

                        let mut glyph_buffer = render_mgr.alloc_buffer();
                        glyph_buffer.replace(&[
                            matrix[0],
                            matrix[1],
                            matrix[2],
                            matrix[3],
                            matrix[4],
                            matrix[5],
                            matrix[6]
                                + matrix[0]
                                    * (tile_offset_x + glyph_offset_x
                                        - sdf_inset as f32 * font_width_scale)
                                + matrix[3]
                                    * (tile_offset_y + glyph_offset_y
                                        - sdf_inset as f32 * font_height_scale),
                            matrix[7]
                                + matrix[1]
                                    * (tile_offset_x + glyph_offset_x
                                        - sdf_inset as f32 * font_width_scale)
                                + matrix[4]
                                    * (tile_offset_y + glyph_offset_y
                                        - sdf_inset as f32 * font_height_scale),
                            matrix[8],
                            glyph_width,
                            glyph_height,
                            renderer.color.r * tile.fore_color.r,
                            renderer.color.g * tile.fore_color.g,
                            renderer.color.b * tile.fore_color.b,
                            renderer.color.a * tile.fore_color.a,
                            thickness,
                            smoothness,
                            (g.mapping.min().0 as f32) / g.texture.width() as f32,
                            (g.mapping.min().1 as f32) / g.texture.height() as f32,
                            (g.mapping.max().0 as f32) / g.texture.width() as f32,
                            (g.mapping.max().1 as f32) / g.texture.height() as f32,
                        ]);
                        glyph_texture_and_buffers.push((g.texture.handle(), glyph_buffer));
                    }
                }

                let back_shader = &renderer.back_shader;
                let fore_shader = &renderer.fore_shader;
                let mut back_buffer = render_mgr.alloc_buffer();
                back_buffer.replace(back_buffer_content.as_slice());

                let mut r = Renderer::new(&self.renderer_bump);

                r.enqueue(
                    instance_count,
                    2,
                    RenderMode::Trangles,
                    back_shader,
                    |req| {
                        render_mgr.apply_common_shader_input(back_shader, req);

                        // TODO: Add shader type checking logic to alert if types have no match.

                        if let Some(uniform) = back_shader.uniform("camera") {
                            req.uniform_f33(uniform.location, camera_matrix_inverse);
                        }

                        if let Some(attribute) = back_shader.attribute("pos") {
                            req.attribute(
                                attribute.location,
                                &self.alpha_tilemap_back_buffer,
                                0,
                                attribute.ty,
                            );
                        }

                        if let Some(attribute) = back_shader.attribute("transform") {
                            req.attribute_per_instance(
                                attribute.location,
                                &back_buffer,
                                0,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = back_shader.attribute("size") {
                            req.attribute_per_instance(
                                attribute.location,
                                &back_buffer,
                                (size_of::<f32>() * 9) as _,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = back_shader.attribute("color") {
                            req.attribute_per_instance(
                                attribute.location,
                                &back_buffer,
                                (size_of::<f32>() * 11) as _,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = back_shader.attribute("tint_color") {
                            req.attribute_per_instance(
                                attribute.location,
                                &back_buffer,
                                (size_of::<f32>() * 15) as _,
                                attribute.ty,
                            );
                        }
                    },
                );
                buffers.push(back_buffer);

                for (texture, buffer) in glyph_texture_and_buffers {
                    r.enqueue(1, 2, RenderMode::Trangles, fore_shader, |req| {
                        render_mgr.apply_common_shader_input(fore_shader, req);

                        // TODO: Add shader type checking logic to alert if types have no match.

                        if let Some(uniform) = fore_shader.uniform("camera") {
                            req.uniform_f33(uniform.location, camera_matrix_inverse);
                        }
                        if let Some(uniform) = fore_shader.uniform("glyph") {
                            req.uniform_texture_raw(uniform.location, texture);
                        }

                        if let Some(attribute) = fore_shader.attribute("pos") {
                            req.attribute(attribute.location, &self.glyph_buffer, 0, attribute.ty);
                        }
                        if let Some(attribute) = fore_shader.attribute("uv") {
                            req.attribute(
                                attribute.location,
                                &self.glyph_buffer,
                                (size_of::<f32>() * 2) as _,
                                attribute.ty,
                            );
                        }

                        if let Some(attribute) = fore_shader.attribute("transform") {
                            req.attribute_per_instance(
                                attribute.location,
                                &buffer,
                                0,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = fore_shader.attribute("size") {
                            req.attribute_per_instance(
                                attribute.location,
                                &buffer,
                                (size_of::<f32>() * 9) as _,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = fore_shader.attribute("color") {
                            req.attribute_per_instance(
                                attribute.location,
                                &buffer,
                                (size_of::<f32>() * 11) as _,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = fore_shader.attribute("thickness") {
                            req.attribute_per_instance(
                                attribute.location,
                                &buffer,
                                (size_of::<f32>() * 15) as _,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = fore_shader.attribute("smoothness") {
                            req.attribute_per_instance(
                                attribute.location,
                                &buffer,
                                (size_of::<f32>() * 16) as _,
                                attribute.ty,
                            );
                        }
                        if let Some(attribute) = fore_shader.attribute("uv_rect") {
                            req.attribute_per_instance(
                                attribute.location,
                                &buffer,
                                (size_of::<f32>() * 17) as _,
                                attribute.ty,
                            );
                        }
                    });
                    buffers.push(buffer);
                }
                renderers.push((renderer.order, r));
            }

            renderers.sort_unstable_by_key(|(order, _)| *order);

            for (_, renderer) in renderers {
                renderer.flush();
            }

            for buffer in buffers {
                render_mgr.dealloc_buffer(buffer);
            }
        }
    }
}
