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

pub(super) struct RenderDims {
    pub(super) min_aspect_ratio: f64,
    pub(super) max_aspect_ratio: f64,
    pub(super) app_dims: (f64, f64),
    pub(super) full_screen_dims: (u32, u32),
    pub(super) tiled_fbo_dims: (u32, u32),
    pub(super) used_screen_dims: (u32, u32),
    pub(super) app_pixel_scalar: f64,
}

impl RenderDims {
    fn new(min_aspect_ratio: f64, max_aspect_ratio: f64, app_height: f64, full_screen_dims: (u32, u32)) -> RenderDims {
        let ratio = full_screen_dims.0 as f64 / full_screen_dims.1 as f64;
        let used_screen_dims = if ratio < min_aspect_ratio {
            let mut h = (full_screen_dims.0 as f64 / min_aspect_ratio).floor() as u32;
            if (full_screen_dims.1 - h) % 2 != 0 { h -= 1; }
            (full_screen_dims.0, h)
        } else if ratio > max_aspect_ratio {
            let mut w = (full_screen_dims.1 as f64 * max_aspect_ratio).floor() as u32;
            if (full_screen_dims.0 - w) % 2 != 0 { w -= 1; }
            (w, full_screen_dims.1)
        } else {
            full_screen_dims
        };
        let ratio = used_screen_dims.0 as f64 / used_screen_dims.1 as f64;
        let app_dims = (app_height * ratio, app_height);
        let app_pixel_scalar = if used_screen_dims.1 == full_screen_dims.1 {
            full_screen_dims.1 as f64 / app_dims.1
        } else {
            full_screen_dims.0 as f64 / app_dims.0
        };
        RenderDims {
            min_aspect_ratio, max_aspect_ratio, app_dims, full_screen_dims, used_screen_dims, app_pixel_scalar,
            tiled_fbo_dims: (to_fbo_dim(app_height * max_aspect_ratio), to_fbo_dim(app_height)),
        }
    }

    pub(super) fn set_full_screen_dims(&mut self, screen_dims: (u32, u32)) {
        *self = RenderDims::new(self.min_aspect_ratio, self.max_aspect_ratio, self.app_dims.1, screen_dims);
    }
}

fn to_fbo_dim(app_dim: f64) -> u32 {
    (app_dim - 1e-7).ceil() as u32 + 1
}

pub struct RenderBuffer {
    pub(super) sprite_atlas: Atlas,
    pub(super) tiled_atlas: Atlas,
    pub(super) mode: Mode,
    pub(super) vbo_data: Vec<f32>,
    pub(super) dims: RenderDims,
}

impl RenderBuffer {
    pub fn new(info: &AppInfo, screen_dims: (u32, u32), sprite_atlas: Atlas, tiled_atlas: Atlas) -> RenderBuffer {
        RenderBuffer {
            sprite_atlas,
            tiled_atlas,
            mode: Mode::Sprite,
            vbo_data: Vec::new(),
            dims: RenderDims::new(info.min_aspect_ratio, info.max_aspect_ratio, info.dims.app_height, screen_dims),
        }
    }

    fn change_mode(&mut self, r: &mut CoreRenderer, mode: Mode) {
        if mode != self.mode {
            self.flush(r);
            self.mode = mode;
        }
    }

    pub(super) fn flush(&mut self, r: &mut CoreRenderer) {
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

    pub(super) fn append_sprite(&mut self, r: &mut CoreRenderer, affine: &Affine, sprite_id: u16, flash_ratio: f64) {
        self.change_mode(r, Mode::Sprite);
        vbo_packer::append_sprite(self, affine, sprite_id, flash_ratio);
    }

    pub(super) fn append_tile(&mut self, r: &mut CoreRenderer, camera: (f64, f64), affine: &Affine, tile_id: u16) {
        self.change_mode(r, Mode::Tiled(camera));
        vbo_packer::append_tile(self, affine, tile_id);
    }
}
