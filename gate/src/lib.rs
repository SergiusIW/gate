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
//! * Support for displaying text
//! * Adding XBox controller input
//! * Generating enums/handles for user-specific assets, and loading those assets
//! * Handling game save data
//! * Playing looping music that has a one-time intro, without any hiccups in the music
//!   (not sure how I'm going to do this, but it's important to me;
//!   game libraries often seem to overlook this fundamental feature)
//! * New renderer modes with new shaders

#[cfg(not(target_arch = "wasm32"))] extern crate sdl2;
#[cfg(not(target_arch = "wasm32"))] extern crate gl;
extern crate byteorder;

pub mod asset_id;
pub mod renderer;
mod app_info;
mod app_context;
mod input;
mod core;

#[cfg(target_arch = "wasm32")]
pub use crate::core::{wasm_imports, wasm_exports};

pub use crate::app_context::{AppContext, Audio};
pub use crate::input::KeyCode;
pub use crate::app_info::AppInfo;

use crate::asset_id::AppAssetId;
use crate::renderer::Renderer;

const MAX_TIMESTEP: f64 = 1. / 15.;

/// Invoke this in a `main` method to run the `App`.
///
/// Will panic if this method is called more than once.
/// The `AppInfo` is used to specify intiailization parameters for the application.
pub fn run<AS: 'static + AppAssetId, AP: 'static + App<AS>>(info: AppInfo, app: AP) {
    core::run(info, app);
}

/// Trait that a user can implement to specify application behavior, passed into `gate::run(...)`.
pub trait App<A: AppAssetId> {
    /// Invoked when the application is first started, default behavior is a no-op.
    fn start(&mut self, _ctx: &mut AppContext<A>) {}

    /// Advances the app state by a given amount of `seconds` (usually a fraction of a second).
    fn advance(&mut self, seconds: f64, ctx: &mut AppContext<A>);

    /// Invoked when a key or mouse button is pressed down.
    fn key_down(&mut self, key: KeyCode, ctx: &mut AppContext<A>);

    /// Invoked when a key or mouse button is released, default behavior is a no-op.
    fn key_up(&mut self, _key: KeyCode, _ctx: &mut AppContext<A>) {}

    /// Render the app in its current state.
    fn render(&mut self, renderer: &mut Renderer<A>, ctx: &AppContext<A>);
}
