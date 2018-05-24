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
    let affine = affine.post_scale(r.dims.pixel_scalar);
    let flash_ratio = (flash_ratio as f32).max(0.0).min(1.0);

    let pad = (
        0.5 / affine.mat().col_0().len() as f32,
        0.5 / affine.mat().col_1().len() as f32,
    );

    let lt = img_coords.lt;
    let rb = img_coords.rb;
    let lb = (lt.0, rb.1);
    let rt = (rb.0, lt.1);

    let dst_lt = (img_coords.lt.0 - img_coords.anchor.0, -(img_coords.lt.1 - img_coords.anchor.1));
    let dst_rb = (img_coords.rb.0 - img_coords.anchor.0, -(img_coords.rb.1 - img_coords.anchor.1));
    let dst_lb = (dst_lt.0, dst_rb.1);
    let dst_rt = (dst_rb.0, dst_lt.1);

    let affine = affine.post_translate(-0.5 * r.dims.used_native_dims.0 as f64, -0.5 * r.dims.used_native_dims.1 as f64)
                       .post_scale_axes(2.0 / r.dims.native_dims.0 as f64, 2.0 / r.dims.native_dims.1 as f64);
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
    vbo_data.push(0.5 / pad.0);
    vbo_data.push(0.5 / pad.1);
    vbo_data.push(src.0 + pad.0);
    vbo_data.push(src.1 + pad.1);
    vbo_data.push(flash_ratio);
}
