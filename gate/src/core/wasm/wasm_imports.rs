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

//! This module contains methods imported from the gate javascript code for WebAssembly.
//! DO NOT USE DIRECTLY!

use std::os::raw::{c_void, c_int};

extern {
    pub fn gateWasmSetScissor(x: c_int, y: c_int, w: c_int, h: c_int);

    pub fn gateWasmClear(r: f32, g: f32, b: f32);
    pub fn gateWasmDrawSprites(size: usize, data: *const c_void);

    pub fn gateWasmPlaySound(id: c_int);
    pub fn gateWasmLoopMusic(id: c_int);
    pub fn gateWasmStopMusic();

    pub fn gateWasmSpriteAtlasBinSize() -> usize;
    pub fn gateWasmSpriteAtlasBinFill(buffer: *mut c_void);
    pub fn gateWasmTiledAtlasBinSize() -> usize;
    pub fn gateWasmTiledAtlasBinFill(buffer: *mut c_void);

    pub fn gateWasmRequestFullscreen();
    pub fn gateWasmCancelFullscreen();
    pub fn gateWasmIsFullscreen() -> c_int;
}
