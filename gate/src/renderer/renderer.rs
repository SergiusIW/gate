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

//! Contains structs relating to application rendering.
//!
//! Rendering uses OpenGL shaders designed specifically for 2D pixel art,
//! looking crisp at any scale or rotation.

use std::marker::PhantomData;

use ::asset_id::{AppAssetId, IdU16};

use super::geom::Affine;
use super::render_buffer::RenderBuffer;
use super::core_renderer::CoreRenderer;

/// Contains methods for rendering visuals to screen.
///
/// The renderer origin is the center of the screen, with +X meaning "right" and +Y meaning "up".
/// The height of the screen in renderer units ("app pixels") is specified by `AppDims.app_height`.
/// The default scaling of each image is such that one source image pixel equals one "app pixel".
///
/// This struct has functions for entering different rendering "modes".
/// Switching between different modes (or the same mode with different parameters)
/// can be expensive, since it involves flushing graphics data and switching shaders,
/// so try to minimize these switches.
pub struct Renderer<A: AppAssetId> { b: RenderBuffer, c: CoreRenderer, phantom: PhantomData<A> }

impl<A: AppAssetId> Renderer<A> {
    pub(crate) fn new(buffer: RenderBuffer, core_renderer: CoreRenderer) -> Renderer<A> {
        let mut result = Renderer { b: buffer, c: core_renderer, phantom: PhantomData };
        result.set_scissor();
        result
    }

    /// Clears the screen with the given `color` in rgb (red-green-blue) format.
    pub fn clear(&mut self, color: (u8, u8, u8)) {
        self.b.flush(&mut self.c);
        self.c.clear(color);
    }

    /// Enters "sprite mode", for rendering sprites.
    pub fn sprite_mode(&mut self) -> SpriteRenderer<A> {
        SpriteRenderer { r: self }
    }

    /// Enters "tiled mode", for rendering tiles that must line up without any seams.
    ///
    /// `camera_x` and `camera_y` specify the location of the center of the screen
    /// relative to the `TiledRenderer` coordinate system origin.
    /// This may be important to specify if a fractional offset is needed,
    /// because an `Affine` used in `TiledRenderer` must have integer offsets.
    pub fn tiled_mode(&mut self, camera_x: f64, camera_y: f64) -> TiledRenderer<A> {
        TiledRenderer { r: self, camera: (camera_x, camera_y) }
    }

    /// Returns the app width, which is restricted by the app height and the aspect ratio range
    /// specified in `AppInfo`.
    pub fn app_width(&self) -> f64 { self.b.dims.app_dims.0 }

    /// Returns the app height, which is a constant specified in `AppInfo`.
    pub fn app_height(&self) -> f64 { self.b.dims.app_dims.1 }

    pub(crate) fn flush(&mut self) {
        self.b.flush(&mut self.c);
    }

    pub(crate) fn set_screen_dims(&mut self, dims: (u32, u32)) {
        if dims != self.b.dims.full_screen_dims {
            self.b.dims.set_full_screen_dims(dims);
            self.set_scissor();
        }
    }

    fn set_scissor(&mut self) {
        self.c.set_scissor(
            (self.b.dims.full_screen_dims.0 - self.b.dims.used_screen_dims.0) / 2,
            (self.b.dims.full_screen_dims.1 - self.b.dims.used_screen_dims.1) / 2,
            self.b.dims.used_screen_dims.0,
            self.b.dims.used_screen_dims.1,
        );
    }
}

/// A rendering mode for sprites.
pub struct SpriteRenderer<'a, A: AppAssetId + 'a> {
    r: &'a mut Renderer<A>,
}

impl<'a, A: AppAssetId + 'a> SpriteRenderer<'a, A> {
    /// Draws the given `sprite` using the given `affine` transformation from the origin.
    pub fn draw(&mut self, affine: &Affine, sprite: A::Sprite) {
        self.draw_flash(affine, sprite, 0.);
    }

    /// Draws the given `sprite` blended with the color white using the given `affine` transformation from the origin.
    ///
    /// `flash_ratio`, capped between `0.0` and `1.0`, controls how much blending occurs with the
    /// color white (`0.0` means use the image unaltered, `1.0` means use white completely).
    pub fn draw_flash(&mut self, affine: &Affine, sprite: A::Sprite, flash_ratio: f64) {
        self.r.b.append_sprite(&mut self.r.c, affine, sprite.id_u16(), flash_ratio);
    }
}

/// A rendering mode for tiles that must line up without any seams.
pub struct TiledRenderer<'a, A: AppAssetId + 'a> {
    camera: (f64, f64),
    r: &'a mut Renderer<A>,
}

impl<'a, A: AppAssetId + 'a> TiledRenderer<'a, A> {
    /// Draws the given `tile` using the given `affine` transformation from the origin.
    ///
    /// The `affine` transformation must be such that the pixels of the `tile` line up with
    /// the "app pixels" of the `TiledRenderer` exactly.
    /// For example, if the `tile` has an anchor on the intersection of pixel boundaries,
    /// then the `affine` should only use integer translation and scaling (not fractional),
    /// and should only use rotations that are multiples of 90 degrees.
    /// Will panic if this is not the case.
    pub fn draw(&mut self, affine: &Affine, tile: A::Tile) {
        self.r.b.append_tile(&mut self.r.c, self.camera, affine, tile.id_u16());
    }
}
