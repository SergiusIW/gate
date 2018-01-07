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

use std::{ptr, str, mem};
use std::os::raw::{c_void, c_char};

use gl::types::*;
use gl;

use super::shader_util;
use renderer::shaders;

pub struct TiledProgram {
    pub handle: GLuint,
    pub vao: GLuint,
    vs: GLuint,
    fs: GLuint,
    pub uniform_tex: GLint,
    pub fbo: GLuint,
    pub fbo_tex: GLuint,
    pub fbo_tex_dims: (u32, u32),
}

impl TiledProgram {
    pub fn new(fbo_tex_dims: (u32, u32)) -> TiledProgram {
        let vs = shader_util::compile_shader(shaders::VS_TILED_SRC, gl::VERTEX_SHADER);
        let fs = shader_util::compile_shader(shaders::FS_TILED_SRC, gl::FRAGMENT_SHADER);
        let handle = shader_util::link_program(vs, fs);
        let vao = TiledProgram::make_vao(handle);
        let (fbo, fbo_tex) = TiledProgram::make_fbo_and_tex(fbo_tex_dims);
        unsafe {
            TiledProgram {
                handle, vao, vs, fs, fbo, fbo_tex, fbo_tex_dims,
                uniform_tex: gl::GetUniformLocation(handle, c_str!("tex")),
            }
        }
    }

    fn make_vao(program_handle: GLuint) -> GLuint {
        let mut vao = 0;
        unsafe {
            let attrib_vert = gl::GetAttribLocation(program_handle, c_str!("vert"));
            let attrib_vs_tex_vert = gl::GetAttribLocation(program_handle, c_str!("vs_tex_vert"));

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // TODO be consistent with the gl::TRUE/FALSE values in gl::VertexAttribPointer...

            gl::EnableVertexAttribArray(attrib_vert as GLuint);
            gl::VertexAttribPointer(attrib_vert as GLuint, 2, gl::FLOAT, gl::FALSE, 4*mem::size_of::<GLfloat>() as i32, ptr::null());

            gl::EnableVertexAttribArray(attrib_vs_tex_vert as GLuint);
            gl::VertexAttribPointer(attrib_vs_tex_vert as GLuint, 2, gl::FLOAT, gl::TRUE, 4*mem::size_of::<GLfloat>() as i32,
                                    (2 * mem::size_of::<GLfloat>()) as *const c_void);

            gl::BindVertexArray(0);
        }
        vao
    }

    fn make_fbo_and_tex(tex_dims: (u32, u32)) -> (GLuint, GLuint) {
        let mut fbo = 0;
        let mut fbo_tex = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl::GenTextures(1, &mut fbo_tex);

            gl::BindTexture(gl::TEXTURE_2D, fbo_tex);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, tex_dims.0 as GLint, tex_dims.1 as GLint,
                           0, gl::RGBA, gl::UNSIGNED_BYTE, ptr::null());

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);

            gl::BindTexture(gl::TEXTURE_2D, 0);

            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, fbo_tex, 0);

            let draw_buffers = [gl::COLOR_ATTACHMENT0];
            gl::DrawBuffers(1, &draw_buffers as *const GLuint);

            assert!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        (fbo, fbo_tex)
    }
}

impl Drop for TiledProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
            gl::DeleteShader(self.fs);
            gl::DeleteShader(self.vs);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteTextures(1, &self.fbo_tex);
            gl::DeleteFramebuffers(1, &self.fbo);
        }
    }
}
