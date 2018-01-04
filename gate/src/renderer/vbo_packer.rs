// Copyright 2017-2018 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::geom::Affine;
use super::render_buffer::{RenderBuffer, Mode};

pub fn append_sprite(r: &mut RenderBuffer, affine: &Affine, sprite_id: u16, flash_ratio: f64) {
    assert!(r.mode == Mode::Sprite);

    let img_coords = r.sprite_atlas.images[&sprite_id];
    let affine = affine.post_scale(r.dims.app_pixel_scalar);
    let flash_ratio = (flash_ratio as f32).max(0.0).min(1.0);
    let inv_tex_dims = (1.0 / r.sprite_atlas.dims.0, 1.0 / r.sprite_atlas.dims.1);

    let pad = (
        (0.5 / affine.mat().col_0().len()).min(0.499) as f32 * inv_tex_dims.0,
        (0.5 / affine.mat().col_1().len()).min(0.499) as f32 * inv_tex_dims.1,
    );

    let lt = (img_coords.lt.0 * inv_tex_dims.0, img_coords.lt.1 * inv_tex_dims.1);
    let rb = (img_coords.rb.0 * inv_tex_dims.0, img_coords.rb.1 * inv_tex_dims.1);
    let lb = (lt.0, rb.1);
    let rt = (rb.0, lt.1);

    let dst_lt = (img_coords.lt.0 - img_coords.anchor.0, -(img_coords.lt.1 - img_coords.anchor.1));
    let dst_rb = (img_coords.rb.0 - img_coords.anchor.0, -(img_coords.rb.1 - img_coords.anchor.1));
    let dst_lb = (dst_lt.0, dst_rb.1);
    let dst_rt = (dst_rb.0, dst_lt.1);

    let affine = affine.post_scale_axes(2.0 / r.dims.full_screen_dims.0 as f64, 2.0 / r.dims.full_screen_dims.1 as f64);
    let aff_lt = affine.apply_f32(dst_lt);
    let aff_rb = affine.apply_f32(dst_rb);
    let aff_lb = affine.apply_f32(dst_lb);
    let aff_rt = affine.apply_f32(dst_rt);

    let vbo_data = &mut r.vbo_data;
    add_sprite_vertex(vbo_data, pad, flash_ratio, lt, aff_lt);
    add_sprite_vertex(vbo_data, pad, flash_ratio, rt, aff_rt);
    add_sprite_vertex(vbo_data, pad, flash_ratio, lb, aff_lb);
    add_sprite_vertex(vbo_data, pad, flash_ratio, rt, aff_rt);
    add_sprite_vertex(vbo_data, pad, flash_ratio, lb, aff_lb);
    add_sprite_vertex(vbo_data, pad, flash_ratio, rb, aff_rb);
}

fn add_sprite_vertex(vbo_data: &mut Vec<f32>, pad: (f32, f32), flash_ratio: f32, src: (f32, f32), dst: (f32, f32)) {
    vbo_data.push(dst.0);
    vbo_data.push(dst.1);
    vbo_data.push(src.0 - pad.0);
    vbo_data.push(src.1 - pad.1);
    vbo_data.push(src.0 + pad.0);
    vbo_data.push(src.1 + pad.1);
    vbo_data.push(flash_ratio);
}

pub fn append_tile(r: &mut RenderBuffer, affine: &Affine, tile_id: u16) {
    let camera = r.mode.tiled_camera();

    let img_coords = r.tiled_atlas.images[&tile_id];
    assert!(affine.pre_translate(img_coords.anchor.0 as f64, img_coords.anchor.1 as f64).is_int_aligned(),
            "affine transformation must exactly line up with pixels for tiled rendering");

    let inv_tex_dims = (1.0 / r.tiled_atlas.dims.0, 1.0 / r.tiled_atlas.dims.1);

    let lt = (img_coords.lt.0 * inv_tex_dims.0, img_coords.lt.1 * inv_tex_dims.1);
    let rb = (img_coords.rb.0 * inv_tex_dims.0, img_coords.rb.1 * inv_tex_dims.1);
    let lb = (lt.0, rb.1);
    let rt = (rb.0, lt.1);

    let dst_lt = (img_coords.lt.0 - img_coords.anchor.0, -(img_coords.lt.1 - img_coords.anchor.1));
    let dst_rb = (img_coords.rb.0 - img_coords.anchor.0, -(img_coords.rb.1 - img_coords.anchor.1));
    let dst_lb = (dst_lt.0, dst_rb.1);
    let dst_rt = (dst_rb.0, dst_lt.1);

    let fbo_camera = fbo_camera(camera, r.dims.tiled_fbo_dims);
    let affine = affine.post_translate(fbo_camera.0 - camera.0, fbo_camera.1 - camera.1)
                       .post_scale_axes(2.0 / r.dims.tiled_fbo_dims.0 as f64,
                                        2.0 / r.dims.tiled_fbo_dims.1 as f64);
    let aff_lt = affine.apply_f32(dst_lt);
    let aff_rb = affine.apply_f32(dst_rb);
    let aff_lb = affine.apply_f32(dst_lb);
    let aff_rt = affine.apply_f32(dst_rt);

    let vbo_data = &mut r.vbo_data;
    add_tile_vertex(vbo_data, lt, aff_lt);
    add_tile_vertex(vbo_data, rt, aff_rt);
    add_tile_vertex(vbo_data, lb, aff_lb);
    add_tile_vertex(vbo_data, rt, aff_rt);
    add_tile_vertex(vbo_data, lb, aff_lb);
    add_tile_vertex(vbo_data, rb, aff_rb);
}

fn add_tile_vertex(vbo_data: &mut Vec<f32>, src: (f32, f32), dst: (f32, f32)) {
    vbo_data.push(dst.0);
    vbo_data.push(dst.1);
    vbo_data.push(src.0);
    vbo_data.push(src.1);
}

fn fbo_camera(camera: (f64, f64), fbo_tex_dims: (u32, u32)) -> (f64, f64) {
    (fbo_camera_coord(camera.0, fbo_tex_dims.0), fbo_camera_coord(camera.1, fbo_tex_dims.1))
}

fn fbo_camera_coord(mut camera: f64, fbo_dim: u32) -> f64 {
    if fbo_dim % 2 == 1 {
        camera += 0.5;
    }
    camera = camera.fract();
    if camera > 0.5 {
        camera -= 1.0
    } else if camera < -0.5 {
        camera += 1.0
    }
    camera
}

pub fn append_tile_fbo(r: &mut RenderBuffer) {
    let camera = r.mode.tiled_camera();

    let s = (1.0 / r.dims.tiled_fbo_dims.0 as f32, 1.0 / r.dims.tiled_fbo_dims.1 as f32);
    let pad = (
        (0.5 / r.dims.app_pixel_scalar).min(0.499) as f32 * s.0,
        (0.5 / r.dims.app_pixel_scalar).min(0.499) as f32 * s.1,
    );
    let camera = fbo_camera(camera, r.dims.tiled_fbo_dims);
    let camera = (0.5 + s.0 * camera.0 as f32, 0.5 + s.1 * camera.1 as f32);
    let half_dims = (s.0 * r.dims.app_dims.0 as f32 * 0.5, s.1 * r.dims.app_dims.1 as f32 * 0.5);
    let lb = (camera.0 - half_dims.0, camera.1 - half_dims.1);
    let rt = (camera.0 + half_dims.0, camera.1 + half_dims.1);
    let lt = (lb.0, rt.1);
    let rb = (rt.0, lb.1);

    let w_ratio = r.dims.used_screen_dims.0 as f32 / r.dims.full_screen_dims.0 as f32;
    let h_ratio = r.dims.used_screen_dims.1 as f32 / r.dims.full_screen_dims.1 as f32;

    let vbo_data = &mut r.vbo_data;
    add_sprite_vertex(vbo_data, pad, 0.0, lb, (-w_ratio, -h_ratio));
    add_sprite_vertex(vbo_data, pad, 0.0, rb, ( w_ratio, -h_ratio));
    add_sprite_vertex(vbo_data, pad, 0.0, lt, (-w_ratio,  h_ratio));
    add_sprite_vertex(vbo_data, pad, 0.0, rb, ( w_ratio, -h_ratio));
    add_sprite_vertex(vbo_data, pad, 0.0, lt, (-w_ratio,  h_ratio));
    add_sprite_vertex(vbo_data, pad, 0.0, rt, ( w_ratio,  h_ratio));
}
