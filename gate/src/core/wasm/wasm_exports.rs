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

//! This module contains methods exported to the gate javascript code for WebAssembly.
//! DO NOT USE DIRECTLY!

use std::os::raw::{c_int, c_char};

use ::input::{KeyEvent, KeyCode};
use ::renderer::shaders;
use super::{app_runner_is_defined, app_runner_borrow, app_runner_borrow_mut};

#[no_mangle]
pub unsafe extern "C" fn gateWasmInit() {
    app_runner_borrow_mut().init();
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmOnResize(w: c_int, h: c_int) {
    app_runner_borrow_mut().resize((w as u32, h as u32));
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmUpdateAndDraw(time_millis: f64) {
    app_runner_borrow_mut().update_and_draw(time_millis / 1000.0);
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmKeyEvent(code: c_int, down: bool) {
    assert!(code >= 0 && code <= 255);
    let code = KeyCode::from_u8(code as u8).unwrap();
    let event = if down { KeyEvent::Pressed } else { KeyEvent::Released };
    app_runner_borrow_mut().input(event, code);
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmIsAppDefined() -> c_int {
    if app_runner_is_defined() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmMusicCount() -> c_int {
    app_runner_borrow().music_count() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmSoundCount() -> c_int {
    app_runner_borrow().sound_count() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmSpriteVertSrc() -> *const c_char {
    shaders::VS_SPRITE_SRC
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmSpriteFragSrc() -> *const c_char {
    shaders::FS_SPRITE_SRC
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmTiledVertSrc() -> *const c_char {
    shaders::VS_TILED_SRC
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmTiledFragSrc() -> *const c_char {
    shaders::FS_TILED_SRC
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmFromTiledVertSrc() -> *const c_char {
    shaders::VS_FROM_TILED_SRC
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmFromTiledFragSrc() -> *const c_char {
    shaders::FS_FROM_TILED_SRC
}
