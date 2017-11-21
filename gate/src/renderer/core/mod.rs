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

#[macro_use] mod shader_util;
mod sprite_program;
mod sprite_renderer;
mod tiled_program;
mod tiled_renderer;

use sdl2::render::Texture;

use gl::types::*;
use gl;

use app_info::AppInfo;
use renderer::atlas::Atlas;
use renderer::geom::Affine;
use self::sprite_program::SpriteProgram;
use self::tiled_program::TiledProgram;

pub struct Renderer {
    sprite_program: SpriteProgram,
    tiled_program: TiledProgram,
    mode: Mode,
    vbo: GLuint,
    vbo_data: Vec<f32>,
    game_pixel_scalar: f64,
    screen_dims: (u32, u32),
    game_dims: (f64, f64),
}

#[derive(PartialEq, Copy, Clone)]
enum Mode { Sprite, Tiled((f64, f64)) }

impl Mode {
    fn tiled_camera(self) -> (f64, f64) {
        match self {
            Mode::Tiled(camera) => camera,
            _ => panic!("not in tiled mode"),
        }
    }
}

impl Renderer {
    pub fn new(info: &AppInfo, sprites_tex: Texture, tiles_tex: Texture) -> Renderer {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        }

        let game_pixel_scalar = info.dims.window_pixels.1 as f64 / info.dims.app_height as f64;
        let game_dims = (info.dims.window_pixels.0 as f64 / game_pixel_scalar,
                         info.dims.window_pixels.1 as f64 / game_pixel_scalar);

        let sprites_atlas = Atlas::new(sprites_tex, "assets/sprites.atlas", 1).expect("error reading sprite atlas");
        let tiles_atlas = Atlas::new(tiles_tex, "assets/tiles.atlas", 0).expect("error reading tiles atlas");

        Renderer {
            sprite_program: SpriteProgram::new(sprites_atlas),
            tiled_program: TiledProgram::new(tiles_atlas, game_dims),
            mode: Mode::Sprite,
            vbo,
            vbo_data: Vec::new(),
            game_pixel_scalar,
            screen_dims: info.dims.window_pixels,
            game_dims,
        }
    }

    pub fn clear(&mut self, color: (u8, u8, u8)) {
        unsafe {
            gl::ClearColor(color.0 as f32 / 255., color.1 as f32 / 255., color.2 as f32 / 255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn change_mode(&mut self, mode: Mode) {
        if mode != self.mode {
            self.flush();
            self.mode = mode;
        }
    }

    pub fn flush(&mut self) {
        match self.mode {
            Mode::Sprite => sprite_renderer::flush(self),
            Mode::Tiled(_) => tiled_renderer::flush(self),
        }
    }

    pub fn draw_sprite(&mut self, affine: &Affine, sprite_id: u16, flash_ratio: f64) {
        self.change_mode(Mode::Sprite);
        sprite_renderer::draw(self, affine, sprite_id, flash_ratio);
    }

    pub fn draw_tile(&mut self, camera: (f64, f64), affine: &Affine, tile_id: u16) {
        self.change_mode(Mode::Tiled(camera));
        tiled_renderer::draw(self, affine, tile_id);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
