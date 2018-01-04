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
//!
//! #Example
//!
//! ```rust
//! use gate::app_info::{AppInfo, AppDims};
//!
//! let info = AppInfo::builder(AppDims { window_pixels: (500, 500), app_height: 100. })
//!                    .title("My Game")
//!                    .target_fps(30.)
//!                    .print_workload_info()
//!                    .print_gl_info()
//!                    .build();
//! ```

// TODO have default for window_pixels in AppInfo, and remove AppDims struct
/// Specifies window and app dimensions.
#[derive(Clone)]
pub struct AppDims {
    /// Window width and height (respectively), in pixels.
    pub window_pixels: (u32, u32),

    /// Height of the screen in conceptual "app pixels", which defines the units used by the renderers.
    ///
    /// The choice of this is important for the `TiledRenderer` in particular.
    pub app_height: f64,
}

/// A struct for specifying initialization information for running an `App`.
#[derive(Clone)]
pub struct AppInfo {
    pub(crate) dims: AppDims,
    pub(crate) min_aspect_ratio: f64,
    pub(crate) max_aspect_ratio: f64,
    pub(crate) title: &'static str,
    pub(crate) target_fps: f64,
    pub(crate) print_workload_info: bool,
    pub(crate) print_gl_info: bool,
}

impl AppInfo {
    /// Returns a builder, initialized with the required value `AppDims`.
    pub fn builder(dims: AppDims) -> AppInfoBuilder {
        assert!(dims.window_pixels.0 >= 10 && dims.window_pixels.0 <= 3000, "unrealistic window width {}", dims.window_pixels.0);
        assert!(dims.window_pixels.1 >= 10 && dims.window_pixels.1 <= 3000, "unrealistic window height {}", dims.window_pixels.1);
        assert!(dims.app_height >= 1e-30 && dims.app_height <= 3000., "unrealistic app height {}", dims.app_height);

        AppInfoBuilder {
            info: AppInfo {
                dims,
                min_aspect_ratio: 4. / 3.,
                max_aspect_ratio: 16. / 9.,
                title: "untitled app",
                target_fps: 60.,
                print_workload_info: false,
                print_gl_info: false,
            }
        }
    }
}

/// Builder for `AppInfo`, created by `AppInfo::builder()`.
pub struct AppInfoBuilder {
    info: AppInfo
}

impl AppInfoBuilder {
    /// Specifies the minimum and maximum aspect ratio for the game, enforced by
    /// letterboxing/pillarboxing if necessary (default is 4/3 to 16/9).
    pub fn aspect_ratio_range(&mut self, min_ratio: f64, max_ratio: f64) -> &mut Self {
        assert!(0.2 < min_ratio && min_ratio < max_ratio && max_ratio < 5.0, "invalid aspect ratios");
        self.info.min_aspect_ratio = min_ratio;
        self.info.max_aspect_ratio = max_ratio;
        self
    }

    /// Specifies a window title (default is "untitled app").
    pub fn title(&mut self, title: &'static str) -> &mut Self { self.info.title = title; self }

    /// Specifies the target frames-per-second (default is `60.`).
    pub fn target_fps(&mut self, target_fps: f64) -> &mut Self {
        assert!(target_fps > 10. && target_fps < 200., "unrealistic target_fps: {}", target_fps);
        self.info.target_fps = target_fps;
        self
    }

    /// If invoked, workload info will be printed to standard output periodically.
    pub fn print_workload_info(&mut self) -> &mut Self { self.info.print_workload_info = true; self }

    /// If invoked, the OpenGL version info will be printed out at the start of the application.
    pub fn print_gl_info(&mut self) -> &mut Self { self.info.print_gl_info = true; self }

    /// Returns an `AppInfo` made from this builder.
    pub fn build(&mut self) -> AppInfo { self.info.clone() }
}
