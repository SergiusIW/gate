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

//! Structs related to user input.
//!
//! Note: `KeyCode`, an enum for keyboard keys, is currently re-exported from SDL2.

use std::collections::HashSet;

use sdl2::EventPump;
use sdl2::event::Event;

use ::App;
use audio::Audio;
use asset_id::AppAssetId;

pub use sdl2::keyboard::Keycode as KeyCode;

/// Events related to a keyboard key.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyEvent {
    /// Key is pressed down.
    Pressed,
    /// A pressed down key is released.
    Released,
}

pub(crate) struct EventHandler {
    pump: EventPump,
    held_keys: HashSet<KeyCode>,
}

impl EventHandler {
    pub fn new(pump: EventPump) -> EventHandler {
        EventHandler { pump, held_keys: HashSet::new() }
    }

    pub fn process_events<AS: AppAssetId, AP: App<AS>>(&mut self, app: &mut AP, audio: &mut Audio<AS>) -> bool {
        for event in self.pump.poll_iter() {
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown { keycode: Some(keycode), .. } =>
                    if self.held_keys.insert(keycode) {
                        let continuing = app.input(KeyEvent::Pressed, keycode, audio);
                        if !continuing { return false }
                    },
                Event::KeyUp { keycode: Some(keycode), .. } =>
                    if self.held_keys.remove(&keycode) {
                        let continuing = app.input(KeyEvent::Released, keycode, audio);
                        if !continuing { return false }
                    },
                _ => {},
            }
        }
        true
    }
}
