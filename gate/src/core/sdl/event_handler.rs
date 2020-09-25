// Copyright 2017-2020 Matthew D. Michelotti
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

use std::collections::HashSet;
use std::mem;

use super::sdl_imports::*;

use crate::{App, AppContext};
use crate::asset_id::AppAssetId;
use crate::input::KeyCode;
use crate::renderer::Renderer;

pub struct EventHandler {
    held_keys: HashSet<KeyCode>,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler { held_keys: HashSet::new() }
    }

    pub unsafe fn process_events<AS: AppAssetId, AP: App<AS>>(&mut self, app: &mut AP, ctx: &mut AppContext<AS>,
        renderer: &Renderer<AS>) -> bool
    {
        loop {
            let mut event = mem::MaybeUninit::uninit();
            if SDL_PollEvent(event.as_mut_ptr()) != 1 {
                break;
            }
            let event = event.assume_init();
            match event.type_ {
                SDL_QUIT => ctx.close(),
                SDL_KEYDOWN => {
                    if let Some(keycode) = sdl_to_gate_key(event.key.keysym.sym) {
                        if self.held_keys.insert(keycode) {
                            app.key_down(keycode, ctx);
                        }
                    }
                },
                SDL_KEYUP => {
                    if let Some(keycode) = sdl_to_gate_key(event.key.keysym.sym) {
                        if self.held_keys.remove(&keycode) {
                            app.key_up(keycode, ctx);
                        }
                    }
                },
                SDL_MOUSEMOTION => {
                    let event = event.motion;
                    ctx.set_cursor(renderer.to_app_pos(event.x, event.y));
                },
                SDL_MOUSEBUTTONDOWN => {
                    let event = event.button;
                    ctx.set_cursor(renderer.to_app_pos(event.x, event.y));
                    if let Some(keycode) = mouse_button_to_gate_key(event.button) {
                        if self.held_keys.insert(keycode) {
                            app.key_down(keycode, ctx);
                        }
                    }
                },
                SDL_MOUSEBUTTONUP => {
                    let event = event.button;
                    ctx.set_cursor(renderer.to_app_pos(event.x, event.y));
                    if let Some(keycode) = mouse_button_to_gate_key(event.button) {
                        if self.held_keys.remove(&keycode) {
                            app.key_up(keycode, ctx);
                        }
                    }
                },
                _ => {},
            }
            if ctx.take_close_request() { return false; }
        }
        true
    }
}

#[allow(non_upper_case_globals)]
fn sdl_to_gate_key(key: i32) -> Option<KeyCode> {
    // TODO consider using a hashmap...
    match key {
        SDLK_a => Some(KeyCode::A),
        SDLK_b => Some(KeyCode::B),
        SDLK_c => Some(KeyCode::C),
        SDLK_d => Some(KeyCode::D),
        SDLK_e => Some(KeyCode::E),
        SDLK_f => Some(KeyCode::F),
        SDLK_g => Some(KeyCode::G),
        SDLK_h => Some(KeyCode::H),
        SDLK_i => Some(KeyCode::I),
        SDLK_j => Some(KeyCode::J),
        SDLK_k => Some(KeyCode::K),
        SDLK_l => Some(KeyCode::L),
        SDLK_m => Some(KeyCode::M),
        SDLK_n => Some(KeyCode::N),
        SDLK_o => Some(KeyCode::O),
        SDLK_p => Some(KeyCode::P),
        SDLK_q => Some(KeyCode::Q),
        SDLK_r => Some(KeyCode::R),
        SDLK_s => Some(KeyCode::S),
        SDLK_t => Some(KeyCode::T),
        SDLK_u => Some(KeyCode::U),
        SDLK_v => Some(KeyCode::V),
        SDLK_w => Some(KeyCode::W),
        SDLK_x => Some(KeyCode::X),
        SDLK_y => Some(KeyCode::Y),
        SDLK_z => Some(KeyCode::Z),
        SDLK_0 => Some(KeyCode::Num0),
        SDLK_1 => Some(KeyCode::Num1),
        SDLK_2 => Some(KeyCode::Num2),
        SDLK_3 => Some(KeyCode::Num3),
        SDLK_4 => Some(KeyCode::Num4),
        SDLK_5 => Some(KeyCode::Num5),
        SDLK_6 => Some(KeyCode::Num6),
        SDLK_7 => Some(KeyCode::Num7),
        SDLK_8 => Some(KeyCode::Num8),
        SDLK_9 => Some(KeyCode::Num9),
        SDLK_RIGHT => Some(KeyCode::Right),
        SDLK_LEFT => Some(KeyCode::Left),
        SDLK_DOWN => Some(KeyCode::Down),
        SDLK_UP => Some(KeyCode::Up),
        SDLK_RETURN => Some(KeyCode::Return),
        SDLK_SPACE => Some(KeyCode::Space),
        SDLK_BACKSPACE => Some(KeyCode::Backspace),
        SDLK_DELETE => Some(KeyCode::Delete),
        _ => None,
    }
}

fn mouse_button_to_gate_key(button: u8) -> Option<KeyCode> {
    match button as u32 {
        SDL_BUTTON_LEFT => Some(KeyCode::MouseLeft),
        SDL_BUTTON_RIGHT => Some(KeyCode::MouseRight),
        SDL_BUTTON_MIDDLE => Some(KeyCode::MouseMiddle),
        _ => None,
    }
}
