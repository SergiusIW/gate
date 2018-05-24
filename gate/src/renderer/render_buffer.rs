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
pub(super) enum Mode { Sprite }

pub(super) struct RenderDims {
    pub min_dims: (f64, f64),
    pub max_dims: (f64, f64),
    pub tile_width: Option<u32>,
    pub native_dims: (u32, u32),
    pub pixel_scalar: f64,
    pub native_pre_pad: (u32, u32),
    pub used_native_dims: (u32, u32),
    pub dims: (f64, f64),
}

impl RenderDims {
    fn new(min_dims: (f64, f64), max_dims: (f64, f64), tile_width: Option<u32>, native_dims: (u32, u32)) -> RenderDims {
        let mut scalar_from_min = (native_dims.0 as f64 / min_dims.0).min(native_dims.1 as f64 / min_dims.1);
        let mut scalar_from_max = (native_dims.0 as f64 / max_dims.0).max(native_dims.1 as f64 / max_dims.1);
        if let Some(tile_width) = tile_width {
            let tile_width = tile_width as f64;
            scalar_from_min = (scalar_from_min * tile_width).floor() / tile_width;
            scalar_from_max = (scalar_from_max * tile_width).ceil() / tile_width;
        }
        let pixel_scalar = scalar_from_min.min(scalar_from_max).max(1.0);
        let used_native_dims = (
            native_dims.0.min((max_dims.0 * pixel_scalar).floor() as u32),
            native_dims.1.min((max_dims.1 * pixel_scalar).floor() as u32),
        );
        let native_pad = (native_dims.0 - used_native_dims.0, native_dims.1 - used_native_dims.1);
        let native_pre_pad = (native_pad.0 / 2, native_pad.1 / 2);
        let dims = (used_native_dims.0 as f64 / pixel_scalar, used_native_dims.1 as f64 / pixel_scalar);
        RenderDims { min_dims, max_dims, tile_width, native_dims, pixel_scalar, native_pre_pad, used_native_dims, dims }
    }

    pub fn set_native_dims(&mut self, native_dims: (u32, u32)) {
        *self = RenderDims::new(self.min_dims, self.max_dims, self.tile_width, native_dims);
    }

    pub fn to_app_pos(&self, raw_x: i32, raw_y: i32) -> (f64, f64) {
        let raw_y = self.native_dims.1 as i32 - raw_y;
        (
            (raw_x - self.native_pre_pad.0 as i32) as f64 / self.pixel_scalar,
            (raw_y - self.native_pre_pad.1 as i32) as f64 / self.pixel_scalar,
        )
    }
}

pub struct RenderBuffer {
    pub(super) sprite_atlas: Atlas,
    pub(super) mode: Mode,
    pub(super) vbo_data: Vec<f32>,
    pub(super) dims: RenderDims,
}

impl RenderBuffer {
    pub fn new(info: &AppInfo, native_dims: (u32, u32), sprite_atlas: Atlas) -> RenderBuffer {
        RenderBuffer {
            sprite_atlas,
            mode: Mode::Sprite,
            vbo_data: Vec::new(),
            dims: RenderDims::new(info.min_dims, info.max_dims, info.tile_width, native_dims),
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
            }
            self.vbo_data.clear();
        }
    }

    pub(super) fn append_sprite(&mut self, r: &mut CoreRenderer, affine: &Affine, sprite_id: u16, flash_ratio: f64) {
        self.change_mode(r, Mode::Sprite);
        vbo_packer::append_sprite(self, affine, sprite_id, flash_ratio);
    }
}
