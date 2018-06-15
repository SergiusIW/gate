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

#![allow(non_snake_case)]

use std::os::raw::{c_int, c_char};

use ::input::KeyCode;
use ::renderer::shaders;
use super::{app_runner_is_defined, app_runner_borrow, app_runner_borrow_mut };

pub fn gateWasmInit() {
    app_runner_borrow_mut().init();
}

pub fn gateWasmOnResize(w: c_int, h: c_int) {
    app_runner_borrow_mut().resize((w as u32, h as u32));
}

pub fn gateWasmUpdateAndDraw(time_millis: f64, cursor_x: c_int, cursor_y: c_int) -> c_int {
    app_runner_borrow_mut().update_cursor(cursor_x as i32, cursor_y as i32);
    let continuing = app_runner_borrow_mut().update_and_draw(time_millis / 1000.0);
    if continuing { 1 } else { 0 }
}

pub fn gateWasmKeyEvent(code: c_int, down: bool) -> c_int {
    assert!(code >= 0 && code <= 255);
    let code = KeyCode::from_u8(code as u8).unwrap();
    let continuing = app_runner_borrow_mut().input(code, down);
    if continuing { 1 } else { 0 }
}

pub fn gateWasmMouseEvent(cursor_x: c_int, cursor_y: c_int, button: c_int, down: bool) -> c_int {
    app_runner_borrow_mut().update_cursor(cursor_x as i32, cursor_y as i32);
    let code = match button {
        0 => Some(KeyCode::MouseLeft),
        1 => Some(KeyCode::MouseMiddle),
        2 => Some(KeyCode::MouseRight),
        _ => None,
    };
    let continuing = if let Some(code) = code {
        app_runner_borrow_mut().input(code, down)
    } else {
        true
    };
    if continuing { 1 } else { 0 }
}

pub fn gateWasmIsAppDefined() -> c_int {
    if app_runner_is_defined() { 1 } else { 0 }
}

pub fn gateWasmMusicCount() -> c_int {
    app_runner_borrow().music_count() as c_int
}

pub fn gateWasmSoundCount() -> c_int {
    app_runner_borrow().sound_count() as c_int
}

pub fn gateWasmSpriteVertSrc() -> *const c_char {
    shaders::VS_SPRITE_SRC
}

pub fn gateWasmSpriteFragSrc() -> *const c_char {
    shaders::FS_SPRITE_SRC
}

pub fn gateWasmOnRestart() {
    app_runner_borrow_mut().on_restart();
}

/// Macro to be placed in the `main.rs` file for a Gate app.
///
/// Currently, the only use this macro has is to export WASM functions for the app
/// when compiling to the `wasm32-unknown-unknown` target.
#[macro_export]
macro_rules! gate_header {
    () => {
        pub mod gate_wasm_exports {
            use std::os::raw::{c_int, c_char};
            #[no_mangle] pub unsafe extern "C" fn gateWasmInit() {
                ::gate::wasm_exports::gateWasmInit()
            }
            #[no_mangle] pub unsafe extern "C" fn gateWasmOnResize(w: c_int, h: c_int) {
                ::gate::wasm_exports::gateWasmOnResize(w, h)
            }
            #[no_mangle] pub unsafe extern "C" fn gateWasmUpdateAndDraw(time_millis: f64, cursor_x: c_int, cursor_y: c_int) -> c_int {
                ::gate::wasm_exports::gateWasmUpdateAndDraw(time_millis, cursor_x, cursor_y)
            }
            #[no_mangle] pub unsafe extern "C" fn gateWasmKeyEvent(code: c_int, down: bool) -> c_int {
                ::gate::wasm_exports::gateWasmKeyEvent(code, down)
            }
            #[no_mangle] pub unsafe extern "C" fn gateWasmMouseEvent(cursor_x: c_int, cursor_y: c_int, button: c_int, down: bool) -> c_int {
                ::gate::wasm_exports::gateWasmMouseEvent(cursor_x, cursor_y, button, down)
            }
            #[no_mangle] pub unsafe extern "C" fn gateWasmIsAppDefined() -> c_int {
                ::gate::wasm_exports::gateWasmIsAppDefined()
            }
            #[no_mangle] pub unsafe extern "C" fn gateWasmMusicCount() -> c_int {
                ::gate::wasm_exports::gateWasmMusicCount()
            }
            #[no_mangle] pub unsafe extern "C" fn gateWasmSoundCount() -> c_int {
                ::gate::wasm_exports::gateWasmSoundCount()
            }
            #[no_mangle] pub unsafe extern "C" fn gateWasmSpriteVertSrc() -> *const c_char {
                ::gate::wasm_exports::gateWasmSpriteVertSrc()
            }
            #[no_mangle] pub unsafe extern "C" fn gateWasmSpriteFragSrc() -> *const c_char {
                ::gate::wasm_exports::gateWasmSpriteFragSrc()
            }
            #[no_mangle] pub unsafe extern "C" fn gateWasmOnRestart() {
                ::gate::wasm_exports::gateWasmOnRestart()
            }
        }
    };
}
