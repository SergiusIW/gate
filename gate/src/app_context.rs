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
    native_px: f64,
}

impl<A: AppAssetId> AppContext<A> {
    pub(crate) fn new(audio: CoreAudio, dims: (f64, f64), native_px: f64) -> AppContext<A> {
        AppContext {
            audio: Audio { core: audio, phantom: PhantomData },
            dims,
            cursor: (0., 0.),
            close_requested: false,
            native_px,
        }
    }

    pub(crate) fn set_cursor(&mut self, cursor: (f64, f64)) {
        self.cursor = cursor;
        self.bound_cursor();
    }

    pub(crate) fn set_dims(&mut self, dims: (f64, f64), native_px: f64) {
        self.dims = dims;
        self.native_px = native_px;
        self.bound_cursor();
    }

    fn bound_cursor(&mut self) {
        self.cursor = (
            self.cursor.0.max(0.).min(self.dims.0),
            self.cursor.1.max(0.).min(self.dims.1),
        );
    }

    /// Returns the app (width, height), which are restricted by the min/max dimensions
    /// specified in `AppInfo`.
    pub fn dims(&self) -> (f64, f64) { self.dims }

    /// Returns the mouse cursor (x, y) position in app coordinates.
    ///
    /// The x coordinate lies in the range `0` to `self.dims().0`.
    /// The y coordinate lies in the range `0` to `self.dims().1`.
    pub fn cursor(&self) -> (f64, f64) { self.cursor }

    /// Returns the width of a native pixel, measured in "app pixels".
    ///
    /// This value will always be at most 1.
    pub fn native_px(&self) -> f64 { self.native_px }

    /// Convenience method for aligning an `(x, y)` position to the nearest native pixel boundaries.
    ///
    /// This is typically used to align a camera position.
    /// See also `self.native_px()`.
    pub fn native_px_align(&self, x: f64, y: f64) -> (f64, f64) {
        (
            (x / self.native_px).round() * self.native_px,
            (y / self.native_px).round() * self.native_px,
        )
    }

    /// Closes the app entirely.
    ///
    /// When compiling to `wasm32-unknown-unknown`, the app may be resumed after it is closed
    /// via invoking a JavaScript method.
    pub fn close(&mut self) { self.close_requested = true; }

    pub(crate) fn take_close_request(&mut self) -> bool {
        let result = self.close_requested;
        self.close_requested = false;
        result
    }
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
