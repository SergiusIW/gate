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
use std::os::raw::c_int;

use renderer::render_buffer::RenderBuffer;
use ::wasm_imports::*;

pub struct CoreRenderer { }

impl CoreRenderer {
    pub fn new(r: &RenderBuffer) -> CoreRenderer {
        unsafe {
            gateWasmSetTiledFboDims(r.tiled_fbo_dims.0 as c_int, r.tiled_fbo_dims.1 as c_int);
        }
        CoreRenderer { }
    }
}

impl CoreRenderer {
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

    pub(in renderer) fn draw_tiles_to_fbo(&mut self, r: &mut RenderBuffer) {
        unsafe {
            gateWasmDrawTilesToFbo(mem::size_of::<f32>() * r.vbo_data.len(), mem::transmute(&r.vbo_data[0]));
        }
    }

    pub(in renderer) fn draw_tiles_from_fbo(&mut self, r: &mut RenderBuffer) {
        unsafe {
            gateWasmDrawTilesFromFbo(mem::size_of::<f32>() * r.vbo_data.len(), mem::transmute(&r.vbo_data[0]));
        }
    }
}
