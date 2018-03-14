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

use std::collections::HashSet;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode as SdlKeyCode;
use sdl2::mouse::MouseButton as SdlMouseButton;

use ::App;
use ::Audio;
use ::asset_id::AppAssetId;
use ::input::{InputEvent, KeyCode, MouseButton};
use ::renderer::Renderer;

pub struct EventHandler {
    pump: EventPump,
    held_keys: HashSet<KeyCode>,
    held_mouse: HashSet<MouseButton>
}

impl EventHandler {
    pub fn new(pump: EventPump) -> EventHandler {
        EventHandler {
            pump,
            held_keys: HashSet::new(),
            held_mouse: HashSet::new(),
        }
    }

    pub fn process_events<AS: AppAssetId, AP: App<AS>>(&mut self, app: &mut AP, audio: &mut Audio<AS>, renderer: &Renderer<AS>) -> bool {
        for event in self.pump.poll_iter() {
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if let Some(keycode) = sdl_to_gate_key(keycode) {
                        if self.held_keys.insert(keycode) {
                            let continuing = app.input(InputEvent::KeyPressed(keycode), audio);
                            if !continuing { return false }
                        }
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    if let Some(keycode) = sdl_to_gate_key(keycode) {
                        if self.held_keys.remove(&keycode) {
                            let continuing = app.input(InputEvent::KeyReleased(keycode), audio);
                            if !continuing { return false }
                        }
                    }
                },
                Event::MouseButtonDown { x, y, mouse_btn, ..} => {
                    if let Some(btn) = sdl_to_gate_mouse_button(mouse_btn) {
                        if self.held_mouse.insert(btn) {
                            let (x, y) = renderer.window_dims_to_app_dims(x as f64, y as f64);
                            let continuing = app.input(InputEvent::MousePressed(btn, x, y), audio);
                            if !continuing { return false }
                        }
                    }
                },
                Event::MouseButtonUp { x, y, mouse_btn, ..} => {
                    if let Some(btn) = sdl_to_gate_mouse_button(mouse_btn) {
                        if self.held_mouse.remove(&btn) {
                            let (x, y) = renderer.window_dims_to_app_dims(x as f64, y as f64);
                            let continuing = app.input(InputEvent::MouseReleased(btn, x, y), audio);
                            if !continuing { return false }
                        }
                    }
                },
                Event::MouseMotion { x, y, ..} => {
                    let (x, y) = renderer.window_dims_to_app_dims(x as f64, y as f64);
                    let continuing = app.input(InputEvent::MouseMotion(x, y), audio);
                    if !continuing { return false }
                },
                _ => {},
            }
        }
        true
    }
}

fn sdl_to_gate_key(sdl: SdlKeyCode) -> Option<KeyCode> {
    match sdl {
        SdlKeyCode::A => Some(KeyCode::A),
        SdlKeyCode::B => Some(KeyCode::B),
        SdlKeyCode::C => Some(KeyCode::C),
        SdlKeyCode::D => Some(KeyCode::D),
        SdlKeyCode::E => Some(KeyCode::E),
        SdlKeyCode::F => Some(KeyCode::F),
        SdlKeyCode::G => Some(KeyCode::G),
        SdlKeyCode::H => Some(KeyCode::H),
        SdlKeyCode::I => Some(KeyCode::I),
        SdlKeyCode::J => Some(KeyCode::J),
        SdlKeyCode::K => Some(KeyCode::K),
        SdlKeyCode::L => Some(KeyCode::L),
        SdlKeyCode::M => Some(KeyCode::M),
        SdlKeyCode::N => Some(KeyCode::N),
        SdlKeyCode::O => Some(KeyCode::O),
        SdlKeyCode::P => Some(KeyCode::P),
        SdlKeyCode::Q => Some(KeyCode::Q),
        SdlKeyCode::R => Some(KeyCode::R),
        SdlKeyCode::S => Some(KeyCode::S),
        SdlKeyCode::T => Some(KeyCode::T),
        SdlKeyCode::U => Some(KeyCode::U),
        SdlKeyCode::V => Some(KeyCode::V),
        SdlKeyCode::W => Some(KeyCode::W),
        SdlKeyCode::X => Some(KeyCode::X),
        SdlKeyCode::Y => Some(KeyCode::Y),
        SdlKeyCode::Z => Some(KeyCode::Z),
        SdlKeyCode::Num0 => Some(KeyCode::Num0),
        SdlKeyCode::Num1 => Some(KeyCode::Num1),
        SdlKeyCode::Num2 => Some(KeyCode::Num2),
        SdlKeyCode::Num3 => Some(KeyCode::Num3),
        SdlKeyCode::Num4 => Some(KeyCode::Num4),
        SdlKeyCode::Num5 => Some(KeyCode::Num5),
        SdlKeyCode::Num6 => Some(KeyCode::Num6),
        SdlKeyCode::Num7 => Some(KeyCode::Num7),
        SdlKeyCode::Num8 => Some(KeyCode::Num8),
        SdlKeyCode::Num9 => Some(KeyCode::Num9),
        SdlKeyCode::Right => Some(KeyCode::Right),
        SdlKeyCode::Left => Some(KeyCode::Left),
        SdlKeyCode::Down => Some(KeyCode::Down),
        SdlKeyCode::Up => Some(KeyCode::Up),
        SdlKeyCode::Return => Some(KeyCode::Return),
        SdlKeyCode::Space => Some(KeyCode::Space),
        _ => None,
    }
}

fn sdl_to_gate_mouse_button(sdl: SdlMouseButton) -> Option<MouseButton> {
    match sdl {
        SdlMouseButton::Left => Some(MouseButton::Left),
        SdlMouseButton::Middle => Some(MouseButton::Middle),
        SdlMouseButton::Right => Some(MouseButton::Right),
        _ => None,
    }
}
