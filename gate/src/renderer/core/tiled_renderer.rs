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
use super::{Renderer, sprite_renderer};

pub fn draw(r: &mut Renderer, affine: &Affine, tile_id: u16) {
    let camera = r.mode.tiled_camera();

    let img_coords = r.tiled_program.atlas.images[&tile_id];
    assert!(affine.pre_translate(img_coords.anchor.0 as f64, img_coords.anchor.1 as f64).is_int_aligned(),
            "affine transformation must exactly line up with pixels for tiled rendering");

    let inv_tex_dims = (1.0 / r.tiled_program.atlas.dims.0, 1.0 / r.tiled_program.atlas.dims.1);

    let lt = (img_coords.lt.0 * inv_tex_dims.0, img_coords.lt.1 * inv_tex_dims.1);
    let rb = (img_coords.rb.0 * inv_tex_dims.0, img_coords.rb.1 * inv_tex_dims.1);
    let lb = (lt.0, rb.1);
    let rt = (rb.0, lt.1);

    let dst_lt = (img_coords.lt.0 - img_coords.anchor.0, -(img_coords.lt.1 - img_coords.anchor.1));
    let dst_rb = (img_coords.rb.0 - img_coords.anchor.0, -(img_coords.rb.1 - img_coords.anchor.1));
    let dst_lb = (dst_lt.0, dst_rb.1);
    let dst_rt = (dst_rb.0, dst_lt.1);

    let fbo_camera = fbo_camera(camera, r.tiled_program.fbo_tex_dims);
    let affine = affine.post_translate(fbo_camera.0 - camera.0, fbo_camera.1 - camera.1)
                       .post_scale_axes(2.0 / r.tiled_program.fbo_tex_dims.0 as f64,
                                        2.0 / r.tiled_program.fbo_tex_dims.1 as f64);
    let aff_lt = affine.apply_f32(dst_lt);
    let aff_rb = affine.apply_f32(dst_rb);
    let aff_lb = affine.apply_f32(dst_lb);
    let aff_rt = affine.apply_f32(dst_rt);

    let vbo_data = &mut r.vbo_data;
    add_vertex(vbo_data, lt, aff_lt);
    add_vertex(vbo_data, rt, aff_rt);
    add_vertex(vbo_data, lb, aff_lb);
    add_vertex(vbo_data, rt, aff_rt);
    add_vertex(vbo_data, lb, aff_lb);
    add_vertex(vbo_data, rb, aff_rb);
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

fn add_vertex(vbo_data: &mut Vec<f32>, src: (f32, f32), dst: (f32, f32)) {
    vbo_data.push(dst.0);
    vbo_data.push(dst.1);
    vbo_data.push(src.0);
    vbo_data.push(src.1);
}

pub fn flush(r: &mut Renderer) {
    let camera = r.mode.tiled_camera();

    if !r.vbo_data.is_empty() {
        draw_to_fbo(r);
        r.vbo_data.clear();
        load_fbo_vbo_rect(r, camera);
        draw_fbo_to_screen(r);
        r.vbo_data.clear();
    }
}

fn draw_to_fbo(r: &mut Renderer) {
    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, r.tiled_program.fbo);

        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::UseProgram(r.tiled_program.handle);
        gl::Viewport(0, 0, r.tiled_program.fbo_tex_dims.0 as GLint, r.tiled_program.fbo_tex_dims.1 as GLint);

        gl::ActiveTexture(gl::TEXTURE0);
        r.tiled_program.atlas.tex.gl_bind_texture();
        gl::Uniform1i(r.tiled_program.uniform_tex, 0);

        gl::BindVertexArray(r.tiled_program.vao);

        gl::BufferData(gl::ARRAY_BUFFER,
                       (mem::size_of::<GLfloat>() * r.vbo_data.len()) as GLsizeiptr,
                       mem::transmute(&r.vbo_data[0]),
                       gl::STREAM_DRAW);

        gl::DrawArrays(gl::TRIANGLES, 0, r.vbo_data.len() as GLint / 4);

        gl::BindVertexArray(0);
        r.tiled_program.atlas.tex.gl_unbind_texture();
        gl::UseProgram(0);
    }
}

fn load_fbo_vbo_rect(r: &mut Renderer, camera: (f64, f64)) {
    let s = (1.0 / r.tiled_program.fbo_tex_dims.0 as f32, 1.0 / r.tiled_program.fbo_tex_dims.1 as f32);
    let pad = (
        (0.5 / r.game_pixel_scalar).min(0.499) as f32 * s.0,
        (0.5 / r.game_pixel_scalar).min(0.499) as f32 * s.1,
    );
    let camera = fbo_camera(camera, r.tiled_program.fbo_tex_dims);
    let camera = (0.5 + s.0 * camera.0 as f32, 0.5 + s.1 * camera.1 as f32);
    let half_dims = (s.0 * r.game_dims.0 as f32 * 0.5, s.1 * r.game_dims.1 as f32 * 0.5);
    let lb = (camera.0 - half_dims.0, camera.1 - half_dims.1);
    let rt = (camera.0 + half_dims.0, camera.1 + half_dims.1);
    let lt = (lb.0, rt.1);
    let rb = (rt.0, lb.1);

    let vbo_data = &mut r.vbo_data;
    sprite_renderer::add_vertex(vbo_data, pad, 0.0, lb, (-1.0, -1.0));
    sprite_renderer::add_vertex(vbo_data, pad, 0.0, rb, ( 1.0, -1.0));
    sprite_renderer::add_vertex(vbo_data, pad, 0.0, lt, (-1.0,  1.0));
    sprite_renderer::add_vertex(vbo_data, pad, 0.0, rb, ( 1.0, -1.0));
    sprite_renderer::add_vertex(vbo_data, pad, 0.0, lt, (-1.0,  1.0));
    sprite_renderer::add_vertex(vbo_data, pad, 0.0, rt, ( 1.0,  1.0));
}

fn draw_fbo_to_screen(r: &mut Renderer) {
    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        gl::UseProgram(r.sprite_program.handle);

        gl::Viewport(0, 0, r.screen_dims.0 as GLint, r.screen_dims.1 as GLint);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, r.tiled_program.fbo_tex);
        gl::Uniform1i(r.sprite_program.uniform_tex, 0); // binds to GL_TEXTURE0
        gl::Uniform2f(r.sprite_program.uniform_tex_dims,
                      r.tiled_program.fbo_tex_dims.0 as f32, r.tiled_program.fbo_tex_dims.1 as f32);

        gl::BindVertexArray(r.sprite_program.vao);

        gl::BufferData(gl::ARRAY_BUFFER,
                       (mem::size_of::<GLfloat>() * r.vbo_data.len()) as GLsizeiptr,
                       mem::transmute(&r.vbo_data[0]),
                       gl::STREAM_DRAW);

        gl::DrawArrays(gl::TRIANGLES, 0, r.vbo_data.len() as GLint / 7);

        gl::BindVertexArray(0);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        gl::UseProgram(0);
    }
}
