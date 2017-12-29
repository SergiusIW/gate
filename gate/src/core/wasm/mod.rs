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

pub mod wasm_imports;
pub mod wasm_exports;

use std::cell::RefCell;
use std::mem;
use std::io::Cursor;

use ::{App, Audio};
use ::asset_id::AppAssetId;
use ::renderer::Renderer;
use ::app_info::AppInfo;
use ::input::{KeyEvent, KeyCode};
use ::renderer::atlas::Atlas;
use ::renderer::render_buffer::RenderBuffer;
use ::renderer::core_renderer::CoreRenderer;
use self::wasm_imports::*;

pub struct CoreAudio;

// FIXME implement CoreAudio
impl CoreAudio {
    pub fn play_sound(&mut self, _sound: u16) { }
    pub fn loop_music(&mut self, _music: u16) { }
    pub fn stop_music(&mut self) { }
}

trait TraitAppRunner {
    fn init(&mut self);
    fn update_and_draw(&mut self, time_sec: f64);
    fn input(&mut self, event: KeyEvent, key: KeyCode);
}

struct DefaultAppRunner;
impl TraitAppRunner for DefaultAppRunner {
    fn init(&mut self) { panic!("gate::run(...) was not invoked") }
    fn update_and_draw(&mut self, _time_sec: f64) { panic!() }
    fn input(&mut self, _event: KeyEvent, _key: KeyCode) { panic!() }
}

struct StaticAppRunner { r: RefCell<Box<TraitAppRunner>> }

// NOTE: StaticAppRunner is not really safe to access concurrently, this was just the easiest way
//       I could find to make it a static variable without artifically requiring `App` to
//       implement `Send`. Do not access concurrently.
unsafe impl Sync for StaticAppRunner {}

lazy_static! {
    static ref APP_RUNNER: StaticAppRunner = StaticAppRunner { r: RefCell::new(Box::new(DefaultAppRunner {})) };
}

struct AppRunner<AS: AppAssetId, AP: App<AS>> {
    app: AP,
    info: AppInfo,
    renderer: Option<Renderer<AS>>,
    last_time_sec: Option<f64>,
}

impl<AS: AppAssetId, AP: App<AS>> TraitAppRunner for AppRunner<AS, AP> {
    fn init(&mut self) {
        assert!(self.renderer.is_none());

        let mut atlas_buf: Vec<u8>;
        unsafe {
            atlas_buf = vec![0; gateWasmSpriteAtlasBinSize()];
            gateWasmSpriteAtlasBinFill(mem::transmute(&mut atlas_buf[0]));
        }
        let sprite_atlas = Atlas::new_sprite(Cursor::new(atlas_buf));

        unsafe {
            atlas_buf = vec![0; gateWasmTiledAtlasBinSize()];
            gateWasmTiledAtlasBinFill(mem::transmute(&mut atlas_buf[0]));
        }
        let tiled_atlas = Atlas::new_tiled(Cursor::new(atlas_buf));

        let render_buffer = RenderBuffer::new(&self.info, sprite_atlas, tiled_atlas);
        let core_renderer = CoreRenderer::new(&render_buffer);
        self.renderer = Some(Renderer::<AS>::new(render_buffer, core_renderer));
    }

    fn update_and_draw(&mut self, time_sec: f64) {
        let mut audio = Audio::new(CoreAudio { });
        let elapsed = self.last_time_sec.map(|x| time_sec - x).unwrap_or(0.0).max(0.0).min(0.1);
        if elapsed > 0.0 {
            self.app.advance(elapsed, &mut audio);
        }
        self.last_time_sec = Some(time_sec);

        self.app.render(self.renderer.as_mut().unwrap());
        self.renderer.as_mut().unwrap().flush();
    }

    fn input(&mut self, event: KeyEvent, key: KeyCode) {
        let mut audio = Audio::new(CoreAudio { });
        self.app.input(event, key, &mut audio);
    }
}

pub fn run<AS: 'static + AppAssetId, AP: 'static + App<AS>>(info: AppInfo, app: AP) {
    *APP_RUNNER.r.borrow_mut() = Box::new(AppRunner { app, info, renderer: None, last_time_sec: None });
}
