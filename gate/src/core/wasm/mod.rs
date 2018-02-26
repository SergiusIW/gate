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

pub mod wasm_imports;
pub mod wasm_exports;

use std::collections::HashSet;
use std::cell::RefCell;
use std::mem;
use std::io::Cursor;
use std::os::raw::c_int;

use ::{App, Audio};
use ::asset_id::{AppAssetId, IdU16};
use ::renderer::Renderer;
use ::app_info::AppInfo;
use ::input::{InputEvent, KeyCode, MouseButton};
use ::renderer::atlas::Atlas;
use ::renderer::render_buffer::RenderBuffer;
use ::renderer::core_renderer::CoreRenderer;
use self::wasm_imports::*;
use super::mark_app_created_flag;

pub struct CoreAudio;

impl CoreAudio {
    pub fn play_sound(&mut self, id: u16) {
        unsafe {
            gateWasmPlaySound(id as c_int);
        }
    }
    pub fn loop_music(&mut self, id: u16) {
        unsafe {
            gateWasmLoopMusic(id as c_int);
        }
    }
    pub fn stop_music(&mut self) {
        unsafe {
            gateWasmStopMusic();
        }
    }
}

trait TraitAppRunner {
    fn init(&mut self);
    fn resize(&mut self, dims: (u32, u32));
    fn update_and_draw(&mut self, time_sec: f64);
    fn input(&mut self, event: InputEvent);
    fn music_count(&self) -> u16;
    fn sound_count(&self) -> u16;
}

struct DefaultAppRunner;
impl TraitAppRunner for DefaultAppRunner {
    fn init(&mut self) { panic!("gate::run(...) was not invoked") }
    fn resize(&mut self, _dims: (u32, u32)) { panic!("gate::run(...) was not invoked") }
    fn update_and_draw(&mut self, _time_sec: f64) { panic!("gate::run(...) was not invoked") }
    fn input(&mut self, _event: InputEvent) { panic!("gate::run(...) was not invoked") }
    fn music_count(&self) -> u16 { panic!("gate::run(...) was not invoked") }
    fn sound_count(&self) -> u16 { panic!("gate::run(...) was not invoked") }
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
    held_keys: HashSet<KeyCode>,
    held_mouse: HashSet<MouseButton>,
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

        let render_buffer = RenderBuffer::new(&self.info, self.info.window_pixels, sprite_atlas, tiled_atlas);
        let core_renderer = CoreRenderer::new(&render_buffer);
        self.renderer = Some(Renderer::<AS>::new(render_buffer, core_renderer));

        let mut audio = Audio::new(CoreAudio { });
        self.app.start(&mut audio);
    }

    fn resize(&mut self, dims: (u32, u32)) {
        self.renderer.as_mut().unwrap().set_screen_dims(dims);
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

    fn input(&mut self, event: InputEvent) {
        let mut audio = Audio::new(CoreAudio { });

        let success = match event {
            InputEvent::KeyPressed(key) => self.held_keys.insert(key),
            InputEvent::KeyReleased(key) => self.held_keys.remove(&key),
            InputEvent::MousePressed(button, _, _) => self.held_mouse.insert(button),
            InputEvent::MouseReleased(button, _, _) => self.held_mouse.remove(&button),
            InputEvent::MouseMotion(_,_) => true
        };
        if success {
            let event = match event {
                InputEvent::MousePressed(button, x, y) => {
                    let (x, y) = self.renderer.as_ref().unwrap().window_dims_to_app_dims(x, y);
                    InputEvent::MousePressed(button, x, y)
                },
                InputEvent::MouseReleased(button, x, y) => {
                    let (x, y) = self.renderer.as_ref().unwrap().window_dims_to_app_dims(x, y);
                    InputEvent::MouseReleased(button, x, y)
                },
                InputEvent::MouseMotion(x, y) => {
                    let (x, y) = self.renderer.as_ref().unwrap().window_dims_to_app_dims(x, y);
                    InputEvent::MouseMotion(x, y)
                },
                e => e,
            };
            self.app.input(event, &mut audio);
        }
    }

    fn music_count(&self) -> u16 { AS::Music::count() }
    fn sound_count(&self) -> u16 { AS::Sound::count() }
}

pub fn run<AS: 'static + AppAssetId, AP: 'static + App<AS>>(info: AppInfo, app: AP) {
    mark_app_created_flag();
    *APP_RUNNER.r.borrow_mut() = Box::new(AppRunner {
        app, info,
        renderer: None,
        last_time_sec: None,
        held_keys: HashSet::new(),
        held_mouse: HashSet::new(),
    });
}
