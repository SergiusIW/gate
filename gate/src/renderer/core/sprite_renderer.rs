// Copyright 2017 Matthew D. Michelotti
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

use std::mem;

use gl::types::*;
use gl;

use renderer::Affine;
use super::{Renderer, Mode};

pub fn draw(r: &mut Renderer, affine: &Affine, sprite_id: u16, flash_ratio: f64) {
    assert!(r.mode == Mode::Sprite);

    let img_coords = r.sprite_program.atlas.images[&sprite_id];
    let affine = affine.post_scale(r.game_pixel_scalar);
    let flash_ratio = (flash_ratio as f32).max(0.0).min(1.0);
    let inv_tex_dims = (1.0 / r.sprite_program.atlas.dims.0, 1.0 / r.sprite_program.atlas.dims.1);

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

    let affine = affine.post_scale_axes(2.0 / r.screen_dims.0 as f64, 2.0 / r.screen_dims.1 as f64);
    let aff_lt = affine.apply_f32(dst_lt);
    let aff_rb = affine.apply_f32(dst_rb);
    let aff_lb = affine.apply_f32(dst_lb);
    let aff_rt = affine.apply_f32(dst_rt);

    let vbo_data = &mut r.vbo_data;
    add_vertex(vbo_data, pad, flash_ratio, lt, aff_lt);
    add_vertex(vbo_data, pad, flash_ratio, rt, aff_rt);
    add_vertex(vbo_data, pad, flash_ratio, lb, aff_lb);
    add_vertex(vbo_data, pad, flash_ratio, rt, aff_rt);
    add_vertex(vbo_data, pad, flash_ratio, lb, aff_lb);
    add_vertex(vbo_data, pad, flash_ratio, rb, aff_rb);
}

pub fn add_vertex(vbo_data: &mut Vec<f32>, pad: (f32, f32), flash_ratio: f32, src: (f32, f32), dst: (f32, f32)) {
    vbo_data.push(dst.0);
    vbo_data.push(dst.1);
    vbo_data.push(src.0 - pad.0);
    vbo_data.push(src.1 - pad.1);
    vbo_data.push(src.0 + pad.0);
    vbo_data.push(src.1 + pad.1);
    vbo_data.push(flash_ratio);
}

pub fn flush(r: &mut Renderer) {
    assert!(r.mode == Mode::Sprite);

    if !r.vbo_data.is_empty() {
        unsafe {
            gl::UseProgram(r.sprite_program.handle);

            gl::ActiveTexture(gl::TEXTURE0);
            r.sprite_program.atlas.tex.gl_bind_texture();
            gl::Uniform1i(r.sprite_program.uniform_tex, 0); // binds to GL_TEXTURE0
            gl::Uniform2f(r.sprite_program.uniform_tex_dims,
                          r.sprite_program.atlas.dims.0, r.sprite_program.atlas.dims.1);

            gl::BindVertexArray(r.sprite_program.vao);

            gl::BufferData(gl::ARRAY_BUFFER,
                           (mem::size_of::<GLfloat>() * r.vbo_data.len()) as GLsizeiptr,
                           mem::transmute(&r.vbo_data[0]),
                           gl::STREAM_DRAW);

            gl::DrawArrays(gl::TRIANGLES, 0, r.vbo_data.len() as GLint / 7);

            gl::BindVertexArray(0);
            r.sprite_program.atlas.tex.gl_unbind_texture();
            gl::UseProgram(0);
        }
        r.vbo_data.clear();
    }
}
