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

//! Contains structs relating to application rendering.
//!
//! Rendering uses OpenGL shaders designed specifically for 2D pixel art,
//! looking crisp at any scale or rotation.

#[macro_use] mod macros;
mod geom;
pub(crate) mod atlas;
pub(crate) mod render_buffer;
mod renderer;
mod vbo_packer;
pub(crate) mod core_renderer;
pub(crate) mod shaders;

pub use self::renderer::*;
pub use self::geom::Affine;
