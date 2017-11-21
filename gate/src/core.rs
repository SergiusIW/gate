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

use std::sync::atomic::{AtomicBool, Ordering};
use std::ffi::CStr;
use std::path::Path;
use std::u32;

use sdl2::{self, VideoSubsystem};
use sdl2::video::GLProfile;
use sdl2::video::gl_attr::GLAttr;
use sdl2::image::LoadTexture;
use sdl2::mixer::{INIT_OGG, DEFAULT_CHANNELS, AUDIO_S16LSB};

use gl;
use gl::types::*;

use audio::Audio;
use app_info::AppInfo;
use renderer::Renderer;
use input::{KeyEvent, KeyCode, EventHandler};
use asset_id::AppAssetId;
use app_clock::AppClock;

/// Trait that a user can implement to specify application behavior, passed into `gate::run(...)`.
pub trait App<A: AppAssetId> {
    /// Invoked when the application is first started, default behavior is a no-op.
    fn start(&mut self, _audio: &mut Audio<A>) {}

    /// Advances the app state by a given amount of `seconds` (usually a fraction of a second).
    fn advance(&mut self, seconds: f64, audio: &mut Audio<A>) -> bool;

    /// Invoked when user input is received (currently only keyboard presses/releases).
    fn input(&mut self, event: KeyEvent, key: KeyCode, audio: &mut Audio<A>) -> bool;

    /// Render the app in its current state.
    fn render(&mut self, renderer: &mut Renderer<A>);
}

lazy_static! {
    static ref APP_CREATED: AtomicBool = AtomicBool::new(false);
}

fn mark_app_created_flag() {
    let previously_created = APP_CREATED.swap(true, Ordering::Relaxed);
    assert!(!previously_created, "Cannot construct more than one App.");
}

/// Invoke this in a `main` method to run the `App`.
///
/// Will panic if this method is called more than once.
/// The `AppInfo` is used to specify intiailization parameters for the application.
pub fn run<AS: AppAssetId, AP: App<AS>>(info: AppInfo, mut app: AP) {
    mark_app_created_flag();

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let _sdl_audio = sdl_context.audio().unwrap();
    let _mixer_context = sdl2::mixer::init(INIT_OGG).unwrap();

    init_mixer();
    gl_hints(video.gl_attr());

    let timer = sdl_context.timer().unwrap();
    let mut event_handler = EventHandler::new(sdl_context.event_pump().unwrap());

    let window = video.window(info.title, info.dims.window_pixels.0, info.dims.window_pixels.1)
        .position_centered().opengl()
        .build().unwrap();

    let mut sdl_renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    init_gl(&video);

    let sprites_tex = sdl_renderer.load_texture(Path::new("assets/sprites.png")).unwrap();
    let tiles_tex = sdl_renderer.load_texture(Path::new("assets/tiles.png")).unwrap();
    // TODO need to ensure Nearest-neighbor sampling is used?

    let mut renderer = Renderer::<AS>::new(&info, sprites_tex, tiles_tex);

    gl_error_check();

    let mut audio = Audio::new();

    if info.print_gl_info { print_gl_info(); }

    app.start(&mut audio);

    let mut clock = AppClock::new(timer, &info);

    'main: loop {
        if cfg!(debug_assertions) {
            renderer.clear((255, 0, 255));
        }

        app.render(&mut renderer);
        renderer.flush();
        sdl_renderer.present();
        gl_error_check();

        let elapsed = clock.step();

        if !event_handler.process_events(&mut app, &mut audio) { break }
        if !app.advance(elapsed, &mut audio) { break; }
    }
}

fn init_mixer() {
    sdl2::mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024).unwrap();
    sdl2::mixer::allocate_channels(4);
}

fn gl_hints(gl_attr: GLAttr) {
    // TODO test that this gl_attr code actually does anything
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_flags().debug().set();
    gl_attr.set_context_version(3, 0);
}

fn init_gl(video: &VideoSubsystem) {
    gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
}

fn print_gl_info() {
    println!("OpenGL version: {:?}", gl_get_string(gl::VERSION));
    println!("GLSL version: {:?}", gl_get_string(gl::SHADING_LANGUAGE_VERSION));
    println!("Vendor: {:?}", gl_get_string(gl::VENDOR));
    println!("Renderer: {:?}", gl_get_string(gl::RENDERER));
}

fn gl_get_string<'a>(name: GLenum) -> &'a CStr {
    unsafe {
        CStr::from_ptr(gl::GetString(name) as *const i8)
    }
}

fn gl_error_check() {
    let error = unsafe { gl::GetError() };
    assert!(error == gl::NO_ERROR, "unexpected OpenGL error, code {}", error);
}
