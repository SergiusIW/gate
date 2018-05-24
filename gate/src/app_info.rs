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
///                    .title("My Game")
///                    .target_fps(30.)
///                    .print_workload_info()
///                    .print_gl_info();
/// ```
pub struct AppInfo {
    pub(crate) window_pixels: (u32, u32),
    pub(crate) min_dims: (f64, f64),
    pub(crate) max_dims: (f64, f64),
    pub(crate) title: &'static str,
    pub(crate) target_fps: f64,
    pub(crate) print_workload_info: bool,
    pub(crate) print_gl_info: bool,
}

// FIXME update rustdoc comments relating to app dims...

impl AppInfo {
    pub fn with_max_dims(max_width: f64, max_height: f64) -> AppInfo {
        assert!(max_width >= 1e-30 && max_width <= 3000., "unrealistic max_width: {}", max_width);
        assert!(max_height >= 1e-30 && max_height <= 3000., "unrealistic max_height: {}", max_height);
        AppInfo {
            window_pixels: (800, 600),
            min_dims: (0., 0.),
            max_dims: (max_width, max_height),
            title: "untitled app",
            target_fps: 60.,
            print_workload_info: false,
            print_gl_info: false,
        }
    }

    pub fn min_dims(mut self, min_width: f64, min_height: f64) -> Self {
        assert!(self.min_dims.0 <= self.max_dims.0 && self.min_dims.1 <= self.max_dims.1);
        self.min_dims = (min_width, min_height);
        self
    }

    /// Specifies a window title (default is "untitled app").
    pub fn title(mut self, title: &'static str) -> Self { self.title = title; self }

    /// Specifies the intial width and height of the window (default is width `800` height `600`).
    pub fn window_pixels(mut self, width: u32, height: u32) -> Self {
        assert!(width >= 10 && width <= 3000, "unrealistic window width {}", width);
        assert!(height >= 10 && height <= 3000, "unrealistic window height {}", height);
        self.window_pixels = (width, height);
        self
    }

    /// Specifies the target frames-per-second (default is `60.`).
    pub fn target_fps(mut self, target_fps: f64) -> Self {
        assert!(target_fps > 10. && target_fps < 200., "unrealistic target_fps: {}", target_fps);
        self.target_fps = target_fps;
        self
    }

    /// If invoked, workload info will be printed to standard output periodically.
    pub fn print_workload_info(mut self) -> Self { self.print_workload_info = true; self }

    /// If invoked, the OpenGL version info will be printed out at the start of the application.
    pub fn print_gl_info(mut self) -> Self { self.print_gl_info = true; self }
}
