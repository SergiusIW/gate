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

pub struct FromTiledProgram {
    pub handle: GLuint,
    pub vao: GLuint,
    vs: GLuint,
    fs: GLuint,
    pub uniform_tex: GLint,
    pub uniform_inv_tex_dims: GLint,
    pub uniform_inv_tex_sample_dim: GLint,
}

impl FromTiledProgram {
    pub fn new() -> FromTiledProgram {
        let vs = shader_util::compile_shader(shaders::VS_FROM_TILED_SRC, gl::VERTEX_SHADER);
        let fs = shader_util::compile_shader(shaders::FS_FROM_TILED_SRC, gl::FRAGMENT_SHADER);
        let handle = shader_util::link_program(vs, fs);
        let vao = FromTiledProgram::make_vao(handle);
        unsafe {
            FromTiledProgram {
                handle, vao, vs, fs,
                uniform_tex: gl::GetUniformLocation(handle, c_str!("tex")),
                uniform_inv_tex_dims: gl::GetUniformLocation(handle, c_str!("inv_tex_dims")),
                uniform_inv_tex_sample_dim: gl::GetUniformLocation(handle, c_str!("inv_tex_sample_dim")),
            }
        }
    }

    fn make_vao(program_handle: GLuint) -> GLuint {
        let mut vao = 0;
        unsafe {
            let attrib_vert = gl::GetAttribLocation(program_handle, c_str!("vert"));
            let attrib_vs_tex_vert_rb = gl::GetAttribLocation(program_handle, c_str!("vs_tex_vert_rb"));

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // TODO be consistent with the gl::TRUE/FALSE values in gl::VertexAttribPointer...

            gl::EnableVertexAttribArray(attrib_vert as GLuint);
            gl::VertexAttribPointer(attrib_vert as GLuint, 2, gl::FLOAT, gl::FALSE, 4*mem::size_of::<GLfloat>() as i32, ptr::null());

            gl::EnableVertexAttribArray(attrib_vs_tex_vert_rb as GLuint);
            gl::VertexAttribPointer(attrib_vs_tex_vert_rb as GLuint, 2, gl::FLOAT, gl::TRUE, 4*mem::size_of::<GLfloat>() as i32,
                                    (2 * mem::size_of::<GLfloat>()) as *const c_void);

            gl::BindVertexArray(0);
        }
        vao
    }
}

impl Drop for FromTiledProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
            gl::DeleteShader(self.fs);
            gl::DeleteShader(self.vs);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
