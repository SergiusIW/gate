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

use sdl2_sys as sdl;

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
            if sdl::SDL_PollEvent(event.as_mut_ptr()) != 1 {
                break;
            }
            let event = event.assume_init();
            match event.type_ {
                t if t == sdl::SDL_EventType::SDL_QUIT as u32 => ctx.close(),
                t if t == sdl::SDL_EventType::SDL_KEYDOWN as u32 => {
                    if let Some(keycode) = sdl_to_gate_key(event.key.keysym.sym) {
                        if self.held_keys.insert(keycode) {
                            app.key_down(keycode, ctx);
                        }
                    }
                },
                t if t == sdl::SDL_EventType::SDL_KEYUP as u32 => {
                    if let Some(keycode) = sdl_to_gate_key(event.key.keysym.sym) {
                        if self.held_keys.remove(&keycode) {
                            app.key_up(keycode, ctx);
                        }
                    }
                },
                t if t == sdl::SDL_EventType::SDL_MOUSEMOTION as u32 => {
                    let event = event.motion;
                    ctx.set_cursor(renderer.to_app_pos(event.x, event.y));
                },
                t if t == sdl::SDL_EventType::SDL_MOUSEBUTTONDOWN as u32 => {
                    let event = event.button;
                    ctx.set_cursor(renderer.to_app_pos(event.x, event.y));
                    if let Some(keycode) = mouse_button_to_gate_key(event.button) {
                        if self.held_keys.insert(keycode) {
                            app.key_down(keycode, ctx);
                        }
                    }
                },
                t if t == sdl::SDL_EventType::SDL_MOUSEBUTTONUP as u32 => {
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

fn sdl_to_gate_key(key: i32) -> Option<KeyCode> {
    // TODO consider using a hashmap...
    match key {
        k if k == sdl::SDLK_a as i32 => Some(KeyCode::A),
        k if k == sdl::SDLK_b as i32 => Some(KeyCode::B),
        k if k == sdl::SDLK_c as i32 => Some(KeyCode::C),
        k if k == sdl::SDLK_d as i32 => Some(KeyCode::D),
        k if k == sdl::SDLK_e as i32 => Some(KeyCode::E),
        k if k == sdl::SDLK_f as i32 => Some(KeyCode::F),
        k if k == sdl::SDLK_g as i32 => Some(KeyCode::G),
        k if k == sdl::SDLK_h as i32 => Some(KeyCode::H),
        k if k == sdl::SDLK_i as i32 => Some(KeyCode::I),
        k if k == sdl::SDLK_j as i32 => Some(KeyCode::J),
        k if k == sdl::SDLK_k as i32 => Some(KeyCode::K),
        k if k == sdl::SDLK_l as i32 => Some(KeyCode::L),
        k if k == sdl::SDLK_m as i32 => Some(KeyCode::M),
        k if k == sdl::SDLK_n as i32 => Some(KeyCode::N),
        k if k == sdl::SDLK_o as i32 => Some(KeyCode::O),
        k if k == sdl::SDLK_p as i32 => Some(KeyCode::P),
        k if k == sdl::SDLK_q as i32 => Some(KeyCode::Q),
        k if k == sdl::SDLK_r as i32 => Some(KeyCode::R),
        k if k == sdl::SDLK_s as i32 => Some(KeyCode::S),
        k if k == sdl::SDLK_t as i32 => Some(KeyCode::T),
        k if k == sdl::SDLK_u as i32 => Some(KeyCode::U),
        k if k == sdl::SDLK_v as i32 => Some(KeyCode::V),
        k if k == sdl::SDLK_w as i32 => Some(KeyCode::W),
        k if k == sdl::SDLK_x as i32 => Some(KeyCode::X),
        k if k == sdl::SDLK_y as i32 => Some(KeyCode::Y),
        k if k == sdl::SDLK_z as i32 => Some(KeyCode::Z),
        k if k == sdl::SDLK_0 as i32 => Some(KeyCode::Num0),
        k if k == sdl::SDLK_1 as i32 => Some(KeyCode::Num1),
        k if k == sdl::SDLK_2 as i32 => Some(KeyCode::Num2),
        k if k == sdl::SDLK_3 as i32 => Some(KeyCode::Num3),
        k if k == sdl::SDLK_4 as i32 => Some(KeyCode::Num4),
        k if k == sdl::SDLK_5 as i32 => Some(KeyCode::Num5),
        k if k == sdl::SDLK_6 as i32 => Some(KeyCode::Num6),
        k if k == sdl::SDLK_7 as i32 => Some(KeyCode::Num7),
        k if k == sdl::SDLK_8 as i32 => Some(KeyCode::Num8),
        k if k == sdl::SDLK_9 as i32 => Some(KeyCode::Num9),
        k if k == sdl::SDLK_RIGHT as i32 => Some(KeyCode::Right),
        k if k == sdl::SDLK_LEFT as i32 => Some(KeyCode::Left),
        k if k == sdl::SDLK_DOWN as i32 => Some(KeyCode::Down),
        k if k == sdl::SDLK_UP as i32 => Some(KeyCode::Up),
        k if k == sdl::SDLK_RETURN as i32 => Some(KeyCode::Return),
        k if k == sdl::SDLK_SPACE as i32 => Some(KeyCode::Space),
        k if k == sdl::SDLK_BACKSPACE as i32 => Some(KeyCode::Backspace),
        k if k == sdl::SDLK_DELETE as i32 => Some(KeyCode::Delete),
        _ => None,
    }
}

fn mouse_button_to_gate_key(button: u8) -> Option<KeyCode> {
    match button as u32 {
        sdl::SDL_BUTTON_LEFT => Some(KeyCode::MouseLeft),
        sdl::SDL_BUTTON_RIGHT => Some(KeyCode::MouseRight),
        sdl::SDL_BUTTON_MIDDLE => Some(KeyCode::MouseMiddle),
        _ => None,
    }
}
