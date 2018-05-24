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

mod shader_util;
mod sprite_program;

use std::mem;

use sdl2::render::Texture;

use gl::types::*;
use gl;

use ::renderer::render_buffer::RenderBuffer;
use self::sprite_program::SpriteProgram;

pub struct CoreRenderer {
    vbo: GLuint,
    sprite_program: SpriteProgram,
    sprites_tex: Texture,
}

impl CoreRenderer {
    pub fn new(sprites_tex: Texture) -> CoreRenderer {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        }
        CoreRenderer { vbo, sprites_tex, sprite_program: SpriteProgram::new() }
    }
}

impl CoreRenderer {
    pub(in renderer) fn set_scissor(&mut self, x: u32, y: u32, w: u32, h: u32) {
        unsafe {
            gl::Scissor(x as i32, y as i32, w as i32, h as i32);
        }
    }

    pub(in renderer) fn clear(&mut self, color: (u8, u8, u8)) {
        unsafe {
            gl::Enable(gl::SCISSOR_TEST);
            gl::ClearColor(color.0 as f32 / 255., color.1 as f32 / 255., color.2 as f32 / 255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Disable(gl::SCISSOR_TEST);
        }
    }

    pub(in renderer) fn draw_sprites(&mut self, r: &mut RenderBuffer) {
        unsafe {
            gl::Enable(gl::SCISSOR_TEST);
            gl::UseProgram(self.sprite_program.handle);

            gl::ActiveTexture(gl::TEXTURE0);
            self.sprites_tex.gl_bind_texture();
            gl::Uniform1i(self.sprite_program.uniform_tex, 0); // binds to GL_TEXTURE0
            gl::Uniform2f(self.sprite_program.uniform_inv_tex_dims,
                          1. / r.sprite_atlas.dims.0, 1. / r.sprite_atlas.dims.1);

            gl::BindVertexArray(self.sprite_program.vao);

            gl::BufferData(gl::ARRAY_BUFFER,
                           (mem::size_of::<GLfloat>() * r.vbo_data.len()) as GLsizeiptr,
                           mem::transmute(&r.vbo_data[0]),
                           gl::STREAM_DRAW);

            gl::DrawArrays(gl::TRIANGLES, 0, r.vbo_data.len() as GLint / 7);

            gl::BindVertexArray(0);
            self.sprites_tex.gl_unbind_texture();
            gl::UseProgram(0);
            gl::Disable(gl::SCISSOR_TEST);
        }
        r.vbo_data.clear();
    }
}

impl Drop for CoreRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
