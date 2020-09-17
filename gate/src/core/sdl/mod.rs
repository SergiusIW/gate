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

pub use self::core_audio::CoreAudio;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::fs::File;
use std::io::BufReader;

use sdl2_sys as sdl;

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
use super::mark_app_created_flag;

/// Macro to be placed in the `main.rs` file for a Gate app.
///
/// Currently, the only use this macro has is to export WASM functions for the app
/// when compiling to the `wasm32-unknown-unknown` target.
#[macro_export]
macro_rules! gate_header {
    () => {};
}

pub fn run<AS: AppAssetId, AP: App<AS>>(info: AppInfo, mut app: AP) {
    unsafe {
        mark_app_created_flag();

        #[cfg(target_os = "windows")]
        sdl::SDL_SetHint(c_str!("SDL_RENDER_DRIVER"), c_str!("opengles2"));
        sdl::SDL_Init(sdl::SDL_INIT_VIDEO | sdl::SDL_INIT_AUDIO | sdl::SDL_INIT_TIMER | sdl::SDL_INIT_EVENTS).sdl_check();
        let mix_init = sdl::mixer::Mix_Init(sdl::mixer::MIX_InitFlags_MIX_INIT_OGG as c_int);
        assert!(mix_init == sdl::mixer::MIX_InitFlags_MIX_INIT_OGG as c_int, "failed to initialize OGG mixer suport");
        // let sdl_context = sdl2::init().unwrap();
        // let video = sdl_context.video().unwrap();
        // let _sdl_audio = sdl_context.audio().unwrap();
        // let _mixer_context = mixer_init();

        mixer_setup();
        gl_hints();

        //let timer = sdl_context.timer().unwrap();
        let mut event_handler = EventHandler::new();

        let title = CString::new(info.title.clone()).expect("invalid title");
        let window = sdl::SDL_CreateWindow(
            title.as_ptr(),
            sdl::SDL_WINDOWPOS_CENTERED_MASK as c_int,
            sdl::SDL_WINDOWPOS_CENTERED_MASK as c_int,
            info.window_pixels.0 as c_int,
            info.window_pixels.1 as c_int,
            sdl::SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32 | sdl::SDL_WindowFlags::SDL_WINDOW_OPENGL as u32,
        );
        if window.is_null() {
            panic!("error creating window"); // TODO better error message using SDL_GetError
        }

        // TODO use SDL_RENDERER_PRESENTVSYNC properly instead of trying to time frames...
        let sdl_renderer = sdl::SDL_CreateRenderer(window, -1, sdl::SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32);
        if sdl_renderer.is_null() {
            panic!("error creating renderer"); // TODO better error message using SDL_GetError
        }

        init_gl();

        let mut renderer = build_renderer(&info, sdl_renderer);

        gl_error_check();

        let mut ctx = AppContext::new(CoreAudio::new(AS::Sound::count()), renderer.app_dims(), renderer.native_px());

        if info.print_gl_info { print_gl_info(); }

        app.start(&mut ctx);

        let mut clock = AppClock::new(&info);

        loop {
            gl::ClearColor(0., 0., 0., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let mut screen_dims = (0, 0);
            sdl::SDL_GetWindowSize(window, &mut screen_dims.0, &mut screen_dims.1);
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

//fn mixer_init() {
    // match sdl2::mixer::init(INIT_OGG) {
    //     Ok(ctx) => ctx,
    //     // HACK TODO remove special handling once SDL2 mixer 2.0.3 is released
    //     //           (see https://bugzilla.libsdl.org/show_bug.cgi?id=3929 for details)
    //     Err(ref msg) if msg.as_str() == "OGG support not available" => Sdl2MixerContext,
    //     Err(msg) => panic!("sdl2::mixer::init failed: {}", msg),
    // }
//}

unsafe fn mixer_setup() {
    sdl::mixer::Mix_OpenAudio(44100, sdl::AUDIO_S16LSB as u16, sdl::mixer::MIX_DEFAULT_CHANNELS as c_int, 1024).mix_check();
    let num_channels_requested = 4; // TODO reconsider this limit
    let num_channels_created = sdl::mixer::Mix_AllocateChannels(4);
    assert!(num_channels_requested == num_channels_created);
}

unsafe fn gl_hints() {
    // TODO test that this SetAttribute code actually does anything
    // TODO Reconsider these flags: 3.0 may be lowered, can try PROFILE_ES, debug flag should not be used in release mode, maybe removed entirely
    sdl::SDL_GL_SetAttribute(sdl::SDL_GLattr::SDL_GL_CONTEXT_PROFILE_MASK, sdl::SDL_GLprofile::SDL_GL_CONTEXT_PROFILE_CORE as c_int).sdl_check();
    sdl::SDL_GL_SetAttribute(sdl::SDL_GLattr::SDL_GL_CONTEXT_FLAGS, sdl::SDL_GLcontextFlag::SDL_GL_CONTEXT_DEBUG_FLAG as c_int).sdl_check();
    sdl::SDL_GL_SetAttribute(sdl::SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION, 3).sdl_check();
    sdl::SDL_GL_SetAttribute(sdl::SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION, 0).sdl_check();
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

trait SdlErrorCode {
    fn sdl_check(self);
    fn mix_check(self);
}

impl SdlErrorCode for c_int {
    // TODO make sure to call this on all relevant sdl return values
    fn sdl_check(self) {
        if self != 0 {
            todo!()
        }
    }

    fn mix_check(self) {
        if self != 0 {
            todo!()
        }
    }
}
