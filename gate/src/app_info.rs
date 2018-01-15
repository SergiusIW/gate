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
//! # Example
//!
//! ```rust
//! use gate::app_info::AppInfo;
//!
//! let info = AppInfo::with_app_height(100.)
//!                    .title("My Game")
//!                    .target_fps(30.)
//!                    .print_workload_info()
//!                    .print_gl_info()
//!                    .build();
//! ```

/// A struct for specifying initialization information for running an `App`.
#[derive(Clone)]
pub struct AppInfo {
    pub(crate) app_height: f64,
    pub(crate) window_pixels: (u32, u32),
    pub(crate) min_aspect_ratio: f64,
    pub(crate) max_aspect_ratio: f64,
    pub(crate) title: &'static str,
    pub(crate) target_fps: f64,
    pub(crate) print_workload_info: bool,
    pub(crate) print_gl_info: bool,
}

impl AppInfo {
    /// Returns a builder, initialized with the required value `app_height`.
    ///
    /// The `app_height` is the height of the screen in conceptual "app pixels",
    /// which defines the units used by the renderers.
    /// Even if the window is resized and the aspect ratio changed,
    /// the app height will always remain the same.
    /// The choice of this is important for the `TiledRenderer` in particular.
    pub fn with_app_height(app_height: f64) -> AppInfoBuilder {
        assert!(app_height >= 1e-30 && app_height <= 3000., "unrealistic app height {}", app_height);

        AppInfoBuilder {
            info: AppInfo {
                app_height,
                window_pixels: (800, 600),
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
    /// letterboxing/pillarboxing if necessary (default is `4/3` to `16/9`).
    pub fn aspect_ratio_range(&mut self, min_ratio: f64, max_ratio: f64) -> &mut Self {
        assert!(0.2 < min_ratio && min_ratio < max_ratio && max_ratio < 5.0, "invalid aspect ratios");
        // TODO ensure there is a large enough gap between min_ratio and max_ratio, so that pixel rounding isn't an issue
        self.info.min_aspect_ratio = min_ratio;
        self.info.max_aspect_ratio = max_ratio;
        self
    }

    /// Specifies a window title (default is "untitled app").
    pub fn title(&mut self, title: &'static str) -> &mut Self { self.info.title = title; self }

    /// Specifies the intial width and height of the window (default is width `800` height `600`).
    pub fn window_pixels(&mut self, width: u32, height: u32) -> &mut Self {
        assert!(width >= 10 && width <= 3000, "unrealistic window width {}", width);
        assert!(height >= 10 && height <= 3000, "unrealistic window height {}", height);
        self.info.window_pixels = (width, height);
        self
    }

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
