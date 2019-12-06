// Copyright 2017-2019 Matthew D. Michelotti
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

use crate::asset_id::{AppAssetId, IdU16};
use crate::core::CoreAudio;

/// Context passed to methods in `App`.
pub struct AppContext<A: AppAssetId> {
    /// Audio playback.
    pub audio: Audio<A>,
    dims: (f64, f64),
    cursor: (f64, f64),
    close_requested: bool,
    native_px: f64,
    is_fullscreen: bool,
    desires_fullscreen: bool,
    cookie: Vec<u8>,
    cookie_updated: bool,
}

impl<A: AppAssetId> AppContext<A> {
    pub(crate) fn new(audio: CoreAudio, dims: (f64, f64), native_px: f64) -> AppContext<A> {
        AppContext {
            audio: Audio { core: audio, phantom: PhantomData },
            dims,
            cursor: (0., 0.),
            close_requested: false,
            native_px,
            is_fullscreen: false,
            desires_fullscreen: false,
            cookie: Vec::new(),
            cookie_updated: false,
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

    /// Requests the app to enter fullscreen mode.
    ///
    /// Depending on the target and how this method is invoked, the app may or may not
    /// actually enter fullscreen mode. When compiling to `wasm32-unknown-unknown` and
    /// running in a web browser, fullscreen requests can only be made successfully during
    /// certain user input events, so invoking fullscreen during `App.start` or `App.advance`
    /// will likely fail.
    pub fn request_fullscreen(&mut self) { self.desires_fullscreen = true; }

    /// Requests the app to cancel fullscreen mode.
    pub fn cancel_fullscreen(&mut self) { self.desires_fullscreen = false; }

    /// Checks whether or not the app is currently in fullscreen mode.
    ///
    /// This value will not change immediately after a call to `request_fullscreen` or
    /// `cancel_fullscreen`.
    pub fn is_fullscreen(&self) -> bool { self.is_fullscreen }

    pub(crate) fn desires_fullscreen(&self) -> bool { self.desires_fullscreen }

    pub(crate) fn set_is_fullscreen(&mut self, is_fullscreen: bool) {
        self.is_fullscreen = is_fullscreen;
        self.desires_fullscreen = is_fullscreen;
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

    /// Gets current cookie data.
    ///
    /// NOTE: this API is likely to change change.
    /// Returns an empty array if cookie is not set or if not running in WebAssembly mode.
    pub fn cookie(&self) -> &[u8] {
        &self.cookie
    }

    /// Writes cookie data.
    ///
    /// NOTE: this API is likely to change change.
    /// Cookie can be used as lightweight save data when built in WebAssembly mode.
    /// Only writes persistent cookie data if built in WebAssembly mode.
    /// To use cookies, the readCookie and writeCookie functions must be passed into gate.js.
    pub fn set_cookie(&mut self, cookie: Vec<u8>) {
        assert!(cookie.len() < 700);
        if cookie != self.cookie {
            self.cookie_updated = true;
            self.cookie = cookie;
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn take_cookie_updated_flag(&mut self) -> bool {
        let was_updated = self.cookie_updated;
        self.cookie_updated = false;
        was_updated
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn cookie_buffer(&mut self) -> &mut Vec<u8> {
        &mut self.cookie
    }
}

/// Struct for audio playback.
pub struct Audio<A: AppAssetId> { core: CoreAudio, phantom: PhantomData<A> }

impl<A: AppAssetId> Audio<A> {
    /// Plays the given sound effect once.
    pub fn play_sound(&mut self, sound: A::Sound) { self.core.play_sound(sound.id_u16()); }

    /// Plays the given music once, replacing the currently playing music, if any.
    pub fn play_music(&mut self, music: A::Music) { self.core.play_music(music.id_u16(), false); }

    /// Continually loops the given music, replacing the currently playing music, if any.
    pub fn loop_music(&mut self, music: A::Music) { self.core.play_music(music.id_u16(), true); }

    /// Stops the currently playing music, if any.
    pub fn stop_music(&mut self) { self.core.stop_music(); }
}
