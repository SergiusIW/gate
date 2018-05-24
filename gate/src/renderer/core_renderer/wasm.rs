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

use std::mem;
use std::os::raw::c_int;

use renderer::render_buffer::RenderBuffer;
use ::wasm_imports::*;

pub struct CoreRenderer { }

impl CoreRenderer {
    pub fn new() -> CoreRenderer {
        CoreRenderer { }
    }
}

impl CoreRenderer {
    pub(in renderer) fn set_scissor(&mut self, x: u32, y: u32, w: u32, h: u32) {
        unsafe {
            gateWasmSetScissor(x as c_int, y as c_int, w as c_int, h as c_int);
        }
    }

    pub(in renderer) fn clear(&mut self, color: (u8, u8, u8)) {
        unsafe {
            gateWasmClear(color.0 as f32 / 255., color.1 as f32 / 255., color.2 as f32 / 255.);
        }
    }

    pub(in renderer) fn draw_sprites(&mut self, r: &mut RenderBuffer) {
        unsafe {
            gateWasmDrawSprites(mem::size_of::<f32>() * r.vbo_data.len(), mem::transmute(&r.vbo_data[0]));
        }
    }
}
