// Copyright 2017-2019 Matthew D. Michelotti
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

extern crate gate_build;

use std::path::Path;
use std::env;
use gate_build::AssetPacker;

fn main() {
    let is_wasm = env::var("TARGET").map(|t| t.starts_with("wasm32")).unwrap_or(false);
    let out_dir = env::var("OUT_DIR").unwrap();
    let gen_code_path = Path::new(&out_dir).join("asset_id.rs");

    let assets_dir = if is_wasm { "html" } else { "assets" };
    let mut packer = AssetPacker::new(Path::new(assets_dir));
    packer.cargo_rerun_if_changed();
    packer.sprites(Path::new("src_assets/sprites"));
    packer.music(Path::new("src_assets/music"));
    packer.sounds(Path::new("src_assets/sounds"));
    if is_wasm { packer.gen_javascript_and_html(); }
    packer.gen_asset_id_code(&gen_code_path);
}
