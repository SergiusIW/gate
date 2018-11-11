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

use std::path::PathBuf;

use sdl2::mixer::{self, Music};

pub struct CoreAudio { music: Option<Music<'static>>, sounds: Vec<mixer::Chunk> }

impl CoreAudio {
    pub(crate) fn new(sound_count: u16) -> CoreAudio {
        let sounds: Vec<_> = (0..sound_count)
            .map(|id| PathBuf::from(format!("assets/sound{}.ogg", id)))
            .map(|p| mixer::Chunk::from_file(p).unwrap())
            .collect();
        CoreAudio { sounds, music: None }
    }

    pub fn play_sound(&mut self, sound: u16) {
        mixer::Channel::all().play(&self.sounds[sound as usize], 0).unwrap();
    }

    pub fn play_music(&mut self, music: u16, loops: bool) {
        let music = &format!("assets/music{}.ogg", music);
        self.stop_music();
        let music = mixer::Music::from_file(music).unwrap();
        music.play(if loops { 1_000_000 } else { 1 }).unwrap();
        self.music = Some(music);
    }

    pub fn stop_music(&mut self) {
        if let Some(_) = self.music.take() {
            Music::halt();
        }
    }
}
