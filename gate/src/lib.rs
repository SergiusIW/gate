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

//! Gate is a game development library tailored to 2D pixel-art games.
//!
//! When creating a game, it is good practice to make a layer,
//! specific to one's needs, that separates the
//! game logic from the resource management, rendering, audio, and other interfacing
//! that is needed for a game.
//! "Gate" is the layer that I created for this purpose with my personal game development endeavors,
//! and I decided to make it public.
//! It should be noted that this library was developed for my own personal needs,
//! and is not meant to be a general purpose game development library.
//! This manifests itself mostly with the renderer, which is made specifically for 2D pixel art.
//! If your game has similar needs or you just want to get something going quickly,
//! then this library is for you.
//! If you have slightly different needs, then you can still use this code as a reference point.
//!
//! Users of this crate should create a build script in their project,
//! invoking functionality from the sibling crate "gate_build".
//! This will generate texture atlases and enums to reference assets.
//! See the "gate_build" crate for more details.
//!
//! # Example usage
//!
//! For a full example, see <https://github.com/SergiusIW/gate/tree/master/example>.
//!
//! # Future changes
//!
//! There are a number of new features I am planning to add to Gate in the future.
//! Some of these will involve breaking changes.
//!
//! * Loading assets on the fly
//! * Entering fullscreen mode
//! * Support for displaying text
//! * Adding XBox controller input
//! * Generating enums/handles for user-specific assets, and loading those assets
//! * Handling game save data
//! * Playing looping music that has a one-time intro, without any hiccups in the music
//!   (not sure how I'm going to do this, but it's important to me;
//!   game libraries often seem to overlook this fundamental feature)
//! * Probably some new renderer modes with new shaders

#[cfg(not(target_arch = "wasm32"))] extern crate sdl2;
#[cfg(not(target_arch = "wasm32"))] extern crate gl;
extern crate byteorder;

pub mod asset_id;
pub mod renderer;
pub mod app_info;
pub mod input;
mod core;

pub use core::*;

use std::marker::PhantomData;

use core::CoreAudio;
use ::asset_id::{AppAssetId, IdU16};
use ::input::{KeyEvent, KeyCode};
use ::renderer::Renderer;
use ::app_info::AppInfo;

/// Invoke this in a `main` method to run the `App`.
///
/// Will panic if this method is called more than once.
/// The `AppInfo` is used to specify intiailization parameters for the application.
pub fn run<AS: 'static + AppAssetId, AP: 'static + App<AS>>(info: AppInfo, app: AP) { core::run(info, app); }

/// Trait that a user can implement to specify application behavior, passed into `gate::run(...)`.
pub trait App<A: AppAssetId> {
    /// Invoked when the application is first started, default behavior is a no-op.
    fn start(&mut self, _ctx: &mut AppContext<A>) {}

    /// Advances the app state by a given amount of `seconds` (usually a fraction of a second).
    fn advance(&mut self, seconds: f64, ctx: &mut AppContext<A>);

    /// Invoked when user input is received (currently only keyboard presses/releases).
    fn input(&mut self, event: KeyEvent, key: KeyCode, ctx: &mut AppContext<A>);

    /// Render the app in its current state.
    fn render(&mut self, renderer: &mut Renderer<A>);
}

pub struct AppContext<A: AppAssetId> { core: CoreAudio, close_requested: bool, phantom: PhantomData<A> }

impl<A: AppAssetId> AppContext<A> {
    pub(crate) fn new(audio: CoreAudio) -> AppContext<A> {
        AppContext {
            core: audio,
            close_requested: false,
            phantom: PhantomData,
        }
    }

    /// Plays the given sound effect once.
    pub fn play_sound(&mut self, sound: A::Sound) { self.core.play_sound(sound.id_u16()); }

    /// Continually loops the given music, replacing the currently playing music, if any.
    pub fn loop_music(&mut self, music: A::Music) { self.core.loop_music(music.id_u16()); }

    /// Stops the currently playing music, if any.
    pub fn stop_music(&mut self) { self.core.stop_music(); }

    pub fn close(&mut self) { self.close_requested = true; }
}
