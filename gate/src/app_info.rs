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

//! Contains `AppInfo` (and related structs), a struct for specifying intialization
//! information for running an `App`.

/// A struct for specifying initialization information for running an `App`.
///
/// Methods for setting fields in `AppInfo` are intended to be chained together like
/// the builder pattern.
///
/// # Example
///
/// ```rust
/// use gate::AppInfo;
///
/// let info = AppInfo::with_max_dims(160., 90.)
///                    .min_dims(120., 86.)
///                    .tile_width(16)
///                    .title("My Game")
///                    .target_fps(30.)
///                    .print_workload_info()
///                    .print_gl_info();
/// ```
pub struct AppInfo {
    pub(crate) window_pixels: (u32, u32),
    pub(crate) min_dims: (f64, f64),
    pub(crate) max_dims: (f64, f64),
    pub(crate) tile_width: Option<u32>,
    pub(crate) title: &'static str,
    pub(crate) target_fps: f64,
    pub(crate) print_workload_info: bool,
    pub(crate) print_gl_info: bool,
}

impl AppInfo {
    /// Returns a new `AppInfo`, initialized with the maximum app dimensions.
    ///
    /// These dimensions are specified in conceptual "app pixels",
    /// which defines the units used by the renderers.
    /// Even if a window is resized, this conecptual `max_width` and `max_height`
    /// will never be exceeded.
    /// Max width/height must be at least 1.
    pub fn with_max_dims(max_width: f64, max_height: f64) -> AppInfo {
        assert!(max_width >= 1. && max_width <= 3000., "unrealistic max_width: {}", max_width);
        assert!(max_height >= 1. && max_height <= 3000., "unrealistic max_height: {}", max_height);
        AppInfo {
            window_pixels: (800, 600),
            min_dims: (0., 0.),
            max_dims: (max_width, max_height),
            tile_width: None,
            title: "untitled app",
            target_fps: 60.,
            print_workload_info: false,
            print_gl_info: false,
        }
    }

    /// Specifies the minimum dimensions in "app pixels" (default is 0).
    ///
    /// Even if you want height to be fixed, it is good practice to design the app so that
    /// min_height is slightly less than max_height.
    /// Under normal circumstances, the app dimensions will not fall below these minimum
    /// dimensions, but there are some extreme cases in which it could.
    /// App dimensions will never fall below 1.
    pub fn min_dims(mut self, min_width: f64, min_height: f64) -> Self {
        assert!(self.min_dims.0 <= self.max_dims.0 && self.min_dims.1 <= self.max_dims.1);
        self.min_dims = (min_width, min_height);
        self
    }

    /// Specifies the tile width for meshing tiles.
    ///
    /// If this value is set, the app dimensions are chosen carefully to ensure that
    /// the width of a tile is aligned to native pixels.
    pub fn tile_width(mut self, tile_width: u32) -> Self {
        assert!(tile_width > 0 && tile_width <= 10000, "unrealistic tile_width {}", tile_width);
        self.tile_width = Some(tile_width);
        self
    }

    /// Specifies a window title (default is "untitled app").
    pub fn title(mut self, title: &'static str) -> Self { self.title = title; self }

    /// Specifies the intial native width and height of the window (default is `800` by `600`).
    pub fn native_dims(mut self, width: u32, height: u32) -> Self {
        assert!(width >= 10 && width <= 3000, "unrealistic window width {}", width);
        assert!(height >= 10 && height <= 3000, "unrealistic window height {}", height);
        self.window_pixels = (width, height);
        self
    }

    /// Specifies the target frames-per-second (default is `60.`).
    pub fn target_fps(mut self, target_fps: f64) -> Self {
        assert!(target_fps >= 20. && target_fps < 200., "unrealistic target_fps: {}", target_fps);
        self.target_fps = target_fps;
        self
    }

    /// If invoked, workload info will be printed to standard output periodically.
    pub fn print_workload_info(mut self) -> Self { self.print_workload_info = true; self }

    /// If invoked, the OpenGL version info will be printed out at the start of the application.
    pub fn print_gl_info(mut self) -> Self { self.print_gl_info = true; self }
}
