// Copyright 2017-2020 Matthew D. Michelotti
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

#[cfg(not(target_arch = "wasm32"))]
mod sdl;

#[cfg(not(target_arch = "wasm32"))]
pub use self::sdl::*;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use self::wasm::*;

use std::sync::atomic::{AtomicBool, Ordering};

static APP_CREATED: AtomicBool = AtomicBool::new(false);

fn mark_app_created_flag() {
    let previously_created = APP_CREATED.swap(true, Ordering::Relaxed);
    assert!(!previously_created, "Cannot construct more than one App.");
}
