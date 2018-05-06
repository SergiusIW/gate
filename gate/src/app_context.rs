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

use std::marker::PhantomData;

use asset_id::{AppAssetId, IdU16};
use core::CoreAudio;

/// Context passed to methods in `App`.
pub struct AppContext<A: AppAssetId> {
    /// Audio playback.
    pub audio: Audio<A>,
    dims: (f64, f64),
    cursor: (f64, f64),
    close_requested: bool,
}

impl<A: AppAssetId> AppContext<A> {
    pub(crate) fn new(audio: CoreAudio, dims: (f64, f64)) -> AppContext<A> {
        AppContext {
            audio: Audio { core: audio, phantom: PhantomData },
            dims,
            cursor: (0., 0.),
            close_requested: false,
        }
    }

    pub(crate) fn set_cursor(&mut self, cursor: (f64, f64)) {
        self.cursor = cursor;
        self.bound_cursor();
    }

    pub(crate) fn set_dims(&mut self, dims: (f64, f64)) {
        self.dims = dims;
        self.bound_cursor();
    }

    fn bound_cursor(&mut self) {
        let (half_width, half_height) = (self.dims.0 * 0.5, self.dims.1 * 0.5);
        self.cursor = (
            self.cursor.0.max(-half_width).min(half_width),
            self.cursor.1.max(-half_height).min(half_height),
        );
    }

    /// Returns the app (width, height), which are restricted by the app height and the
    /// aspect ratio range specified in `AppInfo`.
    pub fn dims(&self) -> (f64, f64) { self.dims }

    /// Returns the mouse cursor (x, y) position in app coordinates.
    ///
    /// The x coordinate lies in the range `-0.5 * self.dims().0` to `0.5 * self.dims().0`.
    /// The y coordinate lies in the range `-0.5 * self.dims().1` to `0.5 * self.dims().1`.
    pub fn cursor(&self) -> (f64, f64) { self.cursor }

    /// Closes the app entirely.
    pub fn close(&mut self) { self.close_requested = true; }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn close_requested(&self) -> bool { self.close_requested }
}

/// Struct for audio playback.
pub struct Audio<A: AppAssetId> { core: CoreAudio, phantom: PhantomData<A> }

impl<A: AppAssetId> Audio<A> {
    /// Plays the given sound effect once.
    pub fn play_sound(&mut self, sound: A::Sound) { self.core.play_sound(sound.id_u16()); }

    /// Continually loops the given music, replacing the currently playing music, if any.
    pub fn loop_music(&mut self, music: A::Music) { self.core.loop_music(music.id_u16()); }

    /// Stops the currently playing music, if any.
    pub fn stop_music(&mut self) { self.core.stop_music(); }
}
