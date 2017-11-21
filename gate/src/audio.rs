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

use std::path::PathBuf;
use std::marker::PhantomData;

use sdl2::mixer::{self, Music};

use asset_id::{AppAssetId, IdU16};

/// Struct for audio playback.
pub struct Audio<A: AppAssetId> {
    music: Option<Music<'static>>,
    sounds: Vec<mixer::Chunk>,
    phantom: PhantomData<A>,
}

impl<A: AppAssetId> Audio<A> {
    pub(crate) fn new() -> Audio<A> {
        let sounds: Vec<_> = (0..(A::Sound::count()))
            .map(|id| PathBuf::from(format!("assets/sound{}.ogg", id)))
            .map(|p| mixer::Chunk::from_file(p).unwrap())
            .collect();
        Audio { sounds, music: None, phantom: PhantomData }
    }

    /// Plays the given sound effect once.
    pub fn play_sound(&mut self, sound: A::Sound) {
        mixer::Channel::all().play(&self.sounds[sound.id_u16() as usize], 0).unwrap();
    }

    /// Continually loops the given music, replacing the currently playing music, if any.
    pub fn loop_music(&mut self, music: A::Music) {
        let music = &format!("assets/music{}.ogg", music.id_u16());
        self.stop_music();
        let music = mixer::Music::from_file(music).unwrap();
        music.play(1_000_000).unwrap();
        self.music = Some(music);
    }

    /// Stops the currently playing music, if any.
    pub fn stop_music(&mut self) {
        if let Some(_) = self.music.take() {
            Music::halt();
        }
    }
}
