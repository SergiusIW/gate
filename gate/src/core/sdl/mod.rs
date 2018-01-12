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

mod app_clock;
mod core_audio;
mod event_handler;

pub use self::core_audio::CoreAudio;

use std::ffi::CStr;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use sdl2::{self, VideoSubsystem};
use sdl2::video::GLProfile;
use sdl2::video::gl_attr::GLAttr;
use sdl2::image::LoadTexture;
use sdl2::mixer::{INIT_OGG, DEFAULT_CHANNELS, AUDIO_S16LSB};
use sdl2::render::{Renderer as SdlRenderer};

use gl;
use gl::types::*;

use ::{Audio, App};
use app_info::AppInfo;
use renderer::Renderer;
use renderer::core_renderer::CoreRenderer;
use renderer::render_buffer::RenderBuffer;
use renderer::atlas::Atlas;
use ::asset_id::{AppAssetId, IdU16};
use self::app_clock::AppClock;
use self::event_handler::EventHandler;
use super::mark_app_created_flag;

const MIN_WINDOW_SIZE: u32 = 100;

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

    let window = video.window(info.title, info.window_pixels.0, info.window_pixels.1)
        .position_centered().opengl().resizable()
        .build().unwrap();

    let mut sdl_renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    init_gl(&video);

    let mut renderer = build_renderer(&info, &sdl_renderer);

    gl_error_check();

    let mut audio = Audio::new(CoreAudio::new(AS::Sound::count()));

    if info.print_gl_info { print_gl_info(); }

    app.start(&mut audio);

    let mut clock = AppClock::new(timer, &info);

    'main: loop {
        unsafe {
            gl::ClearColor(0., 0., 0., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let screen_dims = sdl_renderer.window().unwrap().size();
        if screen_dims.0 >= MIN_WINDOW_SIZE && screen_dims.1 >= MIN_WINDOW_SIZE {
            renderer.set_screen_dims(screen_dims);
            app.render(&mut renderer);
            renderer.flush();
        }
        sdl_renderer.present();
        gl_error_check();

        let elapsed = clock.step();

        if !event_handler.process_events(&mut app, &mut audio) { break }
        if !app.advance(elapsed, &mut audio) { break; }
    }
}

fn build_renderer<AS: AppAssetId>(info: &AppInfo, sdl_renderer: &SdlRenderer) -> Renderer<AS> {
    let sprites_atlas = Atlas::new_sprite(BufReader::new(File::open("assets/sprites.atlas").unwrap()));
    let tiles_atlas = Atlas::new_tiled(BufReader::new(File::open("assets/tiles.atlas").unwrap()));
    let render_buffer = RenderBuffer::new(&info, info.window_pixels, sprites_atlas, tiles_atlas);

    let sprites_tex = sdl_renderer.load_texture(Path::new("assets/sprites.png")).unwrap();
    let tiles_tex = sdl_renderer.load_texture(Path::new("assets/tiles.png")).unwrap();
    // TODO need to ensure Nearest-neighbor sampling is used?
    let core_renderer = CoreRenderer::new(&render_buffer, sprites_tex, tiles_tex);

    Renderer::<AS>::new(render_buffer, core_renderer)
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
