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

use super::core_renderer::CoreRenderer;
use super::vbo_packer;

use ::app_info::AppInfo;
use super::atlas::Atlas;
use super::geom::Affine;

#[derive(PartialEq, Copy, Clone)]
pub(super) enum Mode { Sprite, Tiled((f64, f64)) }

impl Mode {
    pub fn tiled_camera(self) -> (f64, f64) {
        match self {
            Mode::Tiled(camera) => camera,
            _ => panic!("not in tiled mode"),
        }
    }
}

fn to_fbo_dim(game_dim: f64) -> u32 {
    (game_dim - 1e-7).ceil() as u32 + 1
}

pub struct RenderBuffer {
    pub(super) sprite_atlas: Atlas,
    pub(super) tiled_atlas: Atlas,
    pub(super) mode: Mode,
    pub(super) vbo_data: Vec<f32>,
    pub(super) game_pixel_scalar: f64,
    pub(super) screen_dims: (u32, u32),
    pub(super) tiled_fbo_dims: (u32, u32),
}

impl RenderBuffer {
    pub fn new(info: &AppInfo, sprite_atlas: Atlas, tiled_atlas: Atlas) -> RenderBuffer {
        RenderBuffer {
            sprite_atlas,
            tiled_atlas,
            mode: Mode::Sprite,
            vbo_data: Vec::new(),
            game_pixel_scalar: info.dims.window_pixels.1 as f64 / info.dims.app_height as f64,
            screen_dims: info.dims.window_pixels,
            tiled_fbo_dims: (to_fbo_dim(info.dims.app_height * info.dims.window_pixels.0 as f64 / info.dims.window_pixels.1 as f64),
                             to_fbo_dim(info.dims.app_height)),
        }
    }

    pub fn game_dims(&self) -> (f64, f64) {
        (self.screen_dims.0 as f64 / self.game_pixel_scalar,
         self.screen_dims.1 as f64 / self.game_pixel_scalar)
    }

    fn change_mode(&mut self, r: &mut CoreRenderer, mode: Mode) {
        if mode != self.mode {
            self.flush(r);
            self.mode = mode;
        }
    }

    pub fn flush(&mut self, r: &mut CoreRenderer) {
        if !self.vbo_data.is_empty() {
            match self.mode {
                Mode::Sprite => r.draw_sprites(self),
                Mode::Tiled(_) => {
                    r.draw_tiles_to_fbo(self);
                    self.vbo_data.clear();
                    vbo_packer::append_tile_fbo(self);
                    r.draw_tiles_from_fbo(self);
                },
            }
            self.vbo_data.clear();
        }
    }

    pub fn append_sprite(&mut self, r: &mut CoreRenderer, affine: &Affine, sprite_id: u16, flash_ratio: f64) {
        self.change_mode(r, Mode::Sprite);
        vbo_packer::append_sprite(self, affine, sprite_id, flash_ratio);
    }

    pub fn append_tile(&mut self, r: &mut CoreRenderer, camera: (f64, f64), affine: &Affine, tile_id: u16) {
        self.change_mode(r, Mode::Tiled(camera));
        vbo_packer::append_tile(self, affine, tile_id);
    }
}
