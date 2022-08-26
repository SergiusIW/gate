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

use std::ffi::CString;
use std::os::raw::c_char;

use super::sdl_imports::*;

// TODO delete audio after use...
// TODO error checks...
pub struct CoreAudio {
    music: Option<*mut Mix_Music>,
    sounds: Vec<*mut Mix_Chunk>,
}

impl CoreAudio {
    pub(crate) unsafe fn new(sound_count: u16) -> CoreAudio {
        let sounds: Vec<_> = (0..sound_count)
            .map(|id| CString::new(format!("assets/sound{}.ogg", id)).unwrap())
            .map(|p| Mix_LoadWAV_RW(SDL_RWFromFile(p.as_ptr(), c_str!("rb")), 0))
            .collect();
        CoreAudio { sounds, music: None }
    }

    pub fn play_sound(&mut self, sound: u16) {
        unsafe {
            Mix_PlayChannelTimed(-1, self.sounds[sound as usize], 0, -1);
        }
    }

    pub fn play_music(&mut self, music: u16, loops: bool) {
        
            self.stop_music();
            let loops = if loops { -1 } else { 1 };
            let music = CString::new(format!("assets/music{}.ogg", music)).unwrap();
            let music = unsafe {Mix_LoadMUS(music.as_ptr())};
            unsafe {Mix_PlayMusic(music, loops)};
            self.music = Some(music);
        
    }

    pub fn stop_music(&mut self) {
        if let Some(music) = self.music.take() {
            unsafe {
                Mix_FreeMusic(music);
            }
        }
    }
}
