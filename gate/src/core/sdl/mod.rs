// Copyright 2017-2020 Matthew D. Michelotti
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
mod sdl_helpers;
mod sdl_imports;

pub use self::core_audio::CoreAudio;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::fs::File;
use std::io::BufReader;

use sdl2_sys as sdl;
use sdl_helpers::*;
use sdl_imports::*;

use gl;
use gl::types::*;

use crate::{AppContext, App};
use crate::app_info::AppInfo;
use crate::renderer::Renderer;
use crate::renderer::core_renderer::CoreRenderer;
use crate::renderer::render_buffer::RenderBuffer;
use crate::renderer::atlas::Atlas;
use crate::asset_id::{AppAssetId, IdU16};
use self::app_clock::AppClock;
use self::event_handler::EventHandler;

/// Macro to be placed in the `main.rs` file for a Gate app.
///
/// Currently, the only use this macro has is to export WASM functions for the app
/// when compiling to the `wasm32-unknown-unknown` target.
#[macro_export]
macro_rules! gate_header {
    () => {};
}

// FIXME broke window resizing again, fix it...

pub fn run<AS, AP, F>(info: AppInfo, app: F) where
    AS: AppAssetId,
    AP: App<AS>,
    F: FnOnce(&mut AppContext<AS>) -> AP
{
    unsafe {
        SDL_SetHint(SDL_HINT_RENDER_DRIVER, c_str!("opengles2"));
        SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_TIMER | SDL_INIT_EVENTS).sdl_check();
        sdl_assert(Mix_Init(MIX_INIT_OGG) == MIX_INIT_OGG);

        Mix_OpenAudio(MIX_DEFAULT_FREQUENCY, MIX_DEFAULT_FORMAT, MIX_DEFAULT_CHANNELS, 1024).sdl_check();
        assert!(Mix_AllocateChannels(16) == 16);

        let mut event_handler = EventHandler::new();

        SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_ES).sdl_check();
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2).sdl_check();
        SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0).sdl_check();

        let title = CString::new(info.title).expect("invalid title");
        let window = SDL_CreateWindow(
            title.as_ptr(),
            SDL_WINDOWPOS_CENTERED_MASK,
            SDL_WINDOWPOS_CENTERED_MASK,
            info.window_pixels.0 as c_int,
            info.window_pixels.1 as c_int,
            SDL_WINDOW_RESIZABLE | SDL_WINDOW_OPENGL,
        ).sdl_check();

        let sdl_renderer = sdl::SDL_CreateRenderer(window, -1, sdl::SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32
            | sdl::SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC as u32);
        if sdl_renderer.is_null() {
            panic!("error creating renderer"); // TODO better error message using SDL_GetError
        }

        init_gl();

        let mut renderer = build_renderer(&info, sdl_renderer);

        gl_error_check();

        let mut ctx = AppContext::new(CoreAudio::new(AS::Sound::count()), renderer.app_dims(), renderer.native_px());

        if info.print_gl_info { print_gl_info(); }

        let mut app = app(&mut ctx);

        let mut clock = AppClock::new();

        loop {
            gl::ClearColor(0., 0., 0., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let mut screen_dims = (0, 0);
            sdl::SDL_GetWindowSize(window, &mut screen_dims.0, &mut screen_dims.1);
            gl::Viewport(0, 0, screen_dims.0, screen_dims.1); // TODO don't do this unless size changes?
            if screen_dims.0 > 0 && screen_dims.1 > 0 {
                renderer.set_screen_dims((screen_dims.0 as u32, screen_dims.1 as u32));
                ctx.set_dims(renderer.app_dims(), renderer.native_px());
                app.render(&mut renderer, &ctx);
                renderer.flush();
            }
            sdl::SDL_RenderPresent(sdl_renderer);
            gl_error_check();

            let elapsed = clock.step();

            match (ctx.is_fullscreen(), ctx.desires_fullscreen()) {
                (false, true) => {
                    let success = sdl::SDL_SetWindowFullscreen(window, sdl::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32) == 0;
                    ctx.set_is_fullscreen(success);
                },
                (true, false) => {
                    let success = sdl::SDL_SetWindowFullscreen(window, 0) == 0;
                    ctx.set_is_fullscreen(!success);
                },
                (false, false) | (true, true) => {},
            }

            let continuing = event_handler.process_events(&mut app, &mut ctx, &renderer);
            if !continuing { break; }
            app.advance(elapsed.min(crate::MAX_TIMESTEP), &mut ctx);
            if ctx.take_close_request() { break; }
        }
    }
}

unsafe fn build_renderer<AS: AppAssetId>(info: &AppInfo, sdl_renderer: *mut sdl::SDL_Renderer) -> Renderer<AS> {
    let sprites_atlas = Atlas::new(BufReader::new(File::open("assets/sprites.atlas").unwrap())).unwrap();
    let render_buffer = RenderBuffer::new(&info, info.window_pixels, sprites_atlas);

    let sprites_tex = sdl::image::IMG_LoadTexture(sdl_renderer, c_str!("assets/sprites.png"));
    if sprites_tex.is_null() {
        panic!("error loading texture"); // TODO better error message
    }

    let (mut tex_w, mut tex_h) = (0., 0.);
    sdl::SDL_GL_BindTexture(sprites_tex, &mut tex_w, &mut tex_h).sdl_check();
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
    sdl::SDL_GL_UnbindTexture(sprites_tex).sdl_check();

    // TODO need to ensure Nearest-neighbor sampling is used?
    let core_renderer = CoreRenderer::new(sprites_tex);

    Renderer::<AS>::new(render_buffer, core_renderer)
}

unsafe fn init_gl() {
    gl::load_with(|name| {
        let name = CString::new(name).unwrap();
        sdl::SDL_GL_GetProcAddress(name.as_ptr())
    });

    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
}

unsafe fn print_gl_info() {
    println!("OpenGL version: {:?}", gl_get_string(gl::VERSION));
    println!("GLSL version: {:?}", gl_get_string(gl::SHADING_LANGUAGE_VERSION));
    println!("Vendor: {:?}", gl_get_string(gl::VENDOR));
    println!("Renderer: {:?}", gl_get_string(gl::RENDERER));
}

unsafe fn gl_get_string<'a>(name: GLenum) -> &'a CStr {
    CStr::from_ptr(gl::GetString(name) as *const i8)
}

unsafe fn gl_error_check() {
    let error = gl::GetError();
    assert!(error == gl::NO_ERROR, "unexpected OpenGL error, code {}", error);
}
