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
//! #Example usage
//!
//! The below example supposes that in the build script we invoked functionality of "gate_build" to
//! pack assets for a sprite `MySprite`, a tile `MyTile`, music `MyMusic`, and a sound `MySound`,
//! and generated enums in the file "asset_id.rs".
//! See the "gate_build" crate for more details.
//!
//! ```rust,ignore
//! extern crate gate;
//!
//! use gate::{App, Audio};
//! use gate::app_info::{AppInfo, AppDims};
//! use gate::input::{KeyEvent, KeyCode};
//! use gate::renderer::{Renderer, Affine};
//!
//! mod asset_id { include!(concat!(env!("OUT_DIR"), "/asset_id.rs")); }
//! use asset_id::{AssetId, SpriteId, TileId, MusicId, SoundId};
//!
//! struct MyGame { angle: f64, delta: f64 }
//!
//! impl App<AssetId> for MyGame {
//!     fn start(&mut self, audio: &mut Audio<AssetId>) {
//!         audio.loop_music(MusicId::MyMusic);
//!     }
//!
//!     fn advance(&mut self, seconds: f64, _: &mut Audio<AssetId>) -> bool {
//!         self.angle += seconds * self.delta;
//!         true // continue the game
//!     }
//!
//!     fn input(&mut self, evt: KeyEvent, key: KeyCode, audio: &mut Audio<AssetId>) -> bool {
//!         if (evt, key) == (KeyEvent::Pressed, KeyCode::Return) {
//!             self.delta *= -1.;
//!             audio.play_sound(SoundId::MySound);
//!         }
//!         true // continue the game
//!     }
//!
//!     fn render(&mut self, renderer: &mut Renderer<AssetId>) {
//!         { // drawing tiles
//!             let mut renderer = renderer.tiled_mode(32., 32.);
//!             for x in 0..5u32 { for y in 0..5u32 {
//!                 let affine = Affine::translate(x as f64 * 16., y as f64 * 16.);
//!                 renderer.draw(&affine, TileId::MyTile);
//!             }}
//!         }
//!         { // drawing sprites
//!             let mut renderer = renderer.sprite_mode();
//!             renderer.draw(&Affine::rotate(self.angle), SpriteId::MySprite);
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let info = AppInfo::builder(AppDims { window_pixels: (500, 500), app_height: 100. })
//!                        .title("My Game").build();
//!     gate::run(info, MyGame { angle: 0., delta: 90_f64.to_radians() });
//! }
//! ```
//!
//! Note that Gate currently depends on SDL2,
//! so you will need to [install SDL2 development libraries](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries)
//! in order to run your game successfully.
//! Gate requires SDL2, SDL2_Image, and SDL2_Mixer,
//! as well as OpenGL version 3.0 or later.
//!
//! #Future changes
//!
//! There are a number of new features I am planning to add to Gate in the future.
//! Some of these will involve breaking changes.
//!
//! * Loading assets on the fly
//! * Changing resolution or fullscreen mode mid-game
//! * Support for displaying text
//! * Adding XBox controller input
//! * Generating enums/handles for user-specific assets, and loading those assets
//! * Handling game save data
//! * Playing looping music that has a one-time intro, without any hiccups in the music
//!   (not sure how I'm going to do this, but it's important to me;
//!   game libraries often seem to overlook this fundamental feature)
//! * Probably some new renderer modes with new shaders

extern crate sdl2;
extern crate gl;
#[macro_use] extern crate lazy_static;
extern crate byteorder;

pub mod asset_id;
pub mod renderer;
pub mod app_info;
mod app_clock;
mod audio;
mod core;
pub mod input;

pub use audio::Audio;
pub use core::{App, run};
