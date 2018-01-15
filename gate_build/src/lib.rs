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

//! Gate-Build contains utilities for packing image atlases and other assets
//! as part of the build script for a Gate application (see the "gate" crate).
//!
//! The `AssetPacker` of Gate-Build should be invoked in a build script in a Gate application.
//! Rust enums are generated to reference the packed assets.
//!
//! # Example build script
//!
//! In the below example, the user should place sprite png files in the "sprites" directory,
//! tile png files in the "tiles" directory, music ogg files in the "music" directory,
//! and sound ogg files in the "sounds" directory.
//!
//! ```rust,no_run
//! extern crate gate_build;
//!
//! use std::path::Path;
//! use std::env;
//! use gate_build::AssetPacker;
//!
//! fn main() {
//!     let out_dir = env::var("OUT_DIR").unwrap();
//!     let gen_code_path = Path::new(&out_dir).join("asset_id.rs");
//!
//!     let mut packer = AssetPacker::new(Path::new("assets"));
//!     packer.cargo_rerun_if_changed();
//!     packer.sprites(Path::new("sprites"));
//!     packer.tiles(Path::new("tiles"));
//!     packer.music(Path::new("music"));
//!     packer.sounds(Path::new("sounds"));
//!     packer.gen_asset_id_code(&gen_code_path);
//! }
//! ```

extern crate image;
extern crate byteorder;
extern crate regex;
#[macro_use] extern crate lazy_static;

mod rect_packer;
mod atlas;
mod asset_packer;
mod html;

pub use asset_packer::AssetPacker;

use std::path::Path;

fn rerun_print(check_rerun_flag: bool, path: &Path) {
    if check_rerun_flag {
        println!("cargo:rerun-if-changed={}", path.to_str().expect("path could not be converted to string"));
    }
}
