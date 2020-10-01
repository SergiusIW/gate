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

#![allow(non_upper_case_globals)]

use sdl2_sys as sdl;
use sdl::image as image;
use sdl::mixer as mix;
use std::os::raw::{c_char, c_int};

pub use sdl::{
    SDL_BUTTON_LEFT,
    SDL_BUTTON_MIDDLE,
    SDL_BUTTON_RIGHT,
    SDL_CreateRenderer,
    SDL_CreateWindow,
    SDL_GetError,
    SDL_GetWindowSize,
    SDL_GL_BindTexture,
    SDL_GL_GetProcAddress,
    SDL_GL_SetAttribute,
    SDL_GL_UnbindTexture,
    SDL_GLattr::{SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_MAJOR_VERSION, SDL_GL_CONTEXT_MINOR_VERSION},
    SDL_Init,
    SDL_INIT_AUDIO,
    SDL_INIT_EVENTS,
    SDL_INIT_TIMER,
    SDL_INIT_VIDEO,
    SDL_PollEvent,
    SDL_Renderer,
    SDL_RenderPresent,
    SDL_SetHint,
    SDL_SetWindowFullscreen,
};

pub const SDL_GL_CONTEXT_PROFILE_ES: c_int = sdl::SDL_GLprofile::SDL_GL_CONTEXT_PROFILE_ES as c_int;
pub const SDL_HINT_RENDER_DRIVER: *const c_char = sdl::SDL_HINT_RENDER_DRIVER as *const u8 as *const c_char;
pub const SDL_KEYDOWN: u32 = sdl::SDL_EventType::SDL_KEYDOWN as u32;
pub const SDL_KEYUP: u32 = sdl::SDL_EventType::SDL_KEYUP as u32;
pub const SDL_MOUSEBUTTONDOWN: u32 = sdl::SDL_EventType::SDL_MOUSEBUTTONDOWN as u32;
pub const SDL_MOUSEBUTTONUP: u32 = sdl::SDL_EventType::SDL_MOUSEBUTTONUP as u32;
pub const SDL_MOUSEMOTION: u32 = sdl::SDL_EventType::SDL_MOUSEMOTION as u32;
pub const SDL_QUIT: u32 = sdl::SDL_EventType::SDL_QUIT as u32;
pub const SDL_RENDERER_ACCELERATED: u32 = sdl::SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32;
pub const SDL_RENDERER_PRESENTVSYNC: u32 = sdl::SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC as u32;
pub const SDL_WINDOW_OPENGL: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_OPENGL as u32;
pub const SDL_WINDOW_RESIZABLE: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32;
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32;
pub const SDL_WINDOWPOS_CENTERED_MASK: c_int = sdl::SDL_WINDOWPOS_CENTERED_MASK as c_int;

pub const SDLK_a: i32 = sdl::SDLK_a as i32;
pub const SDLK_b: i32 = sdl::SDLK_b as i32;
pub const SDLK_c: i32 = sdl::SDLK_c as i32;
pub const SDLK_d: i32 = sdl::SDLK_d as i32;
pub const SDLK_e: i32 = sdl::SDLK_e as i32;
pub const SDLK_f: i32 = sdl::SDLK_f as i32;
pub const SDLK_g: i32 = sdl::SDLK_g as i32;
pub const SDLK_h: i32 = sdl::SDLK_h as i32;
pub const SDLK_i: i32 = sdl::SDLK_i as i32;
pub const SDLK_j: i32 = sdl::SDLK_j as i32;
pub const SDLK_k: i32 = sdl::SDLK_k as i32;
pub const SDLK_l: i32 = sdl::SDLK_l as i32;
pub const SDLK_m: i32 = sdl::SDLK_m as i32;
pub const SDLK_n: i32 = sdl::SDLK_n as i32;
pub const SDLK_o: i32 = sdl::SDLK_o as i32;
pub const SDLK_p: i32 = sdl::SDLK_p as i32;
pub const SDLK_q: i32 = sdl::SDLK_q as i32;
pub const SDLK_r: i32 = sdl::SDLK_r as i32;
pub const SDLK_s: i32 = sdl::SDLK_s as i32;
pub const SDLK_t: i32 = sdl::SDLK_t as i32;
pub const SDLK_u: i32 = sdl::SDLK_u as i32;
pub const SDLK_v: i32 = sdl::SDLK_v as i32;
pub const SDLK_w: i32 = sdl::SDLK_w as i32;
pub const SDLK_x: i32 = sdl::SDLK_x as i32;
pub const SDLK_y: i32 = sdl::SDLK_y as i32;
pub const SDLK_z: i32 = sdl::SDLK_z as i32;
pub const SDLK_0: i32 = sdl::SDLK_0 as i32;
pub const SDLK_1: i32 = sdl::SDLK_1 as i32;
pub const SDLK_2: i32 = sdl::SDLK_2 as i32;
pub const SDLK_3: i32 = sdl::SDLK_3 as i32;
pub const SDLK_4: i32 = sdl::SDLK_4 as i32;
pub const SDLK_5: i32 = sdl::SDLK_5 as i32;
pub const SDLK_6: i32 = sdl::SDLK_6 as i32;
pub const SDLK_7: i32 = sdl::SDLK_7 as i32;
pub const SDLK_8: i32 = sdl::SDLK_8 as i32;
pub const SDLK_9: i32 = sdl::SDLK_9 as i32;
pub const SDLK_RIGHT: i32 = sdl::SDLK_RIGHT as i32;
pub const SDLK_LEFT: i32 = sdl::SDLK_LEFT as i32;
pub const SDLK_DOWN: i32 = sdl::SDLK_DOWN as i32;
pub const SDLK_UP: i32 = sdl::SDLK_UP as i32;
pub const SDLK_RETURN: i32 = sdl::SDLK_RETURN as i32;
pub const SDLK_SPACE: i32 = sdl::SDLK_SPACE as i32;
pub const SDLK_BACKSPACE: i32 = sdl::SDLK_BACKSPACE as i32;
pub const SDLK_DELETE: i32 = sdl::SDLK_DELETE as i32;

pub use mix::{
    Mix_AllocateChannels,
    Mix_Init,
    Mix_OpenAudio,
};

pub const MIX_DEFAULT_CHANNELS: c_int = mix::MIX_DEFAULT_CHANNELS as c_int;
pub const MIX_DEFAULT_FORMAT: u16 = mix::MIX_DEFAULT_FORMAT as u16;
pub const MIX_DEFAULT_FREQUENCY: c_int = mix::MIX_DEFAULT_FREQUENCY as c_int;
pub const MIX_INIT_OGG: c_int = mix::MIX_InitFlags_MIX_INIT_OGG as c_int;

pub use image::{
    IMG_LoadTexture,
};
