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

use super::sdl_imports::*;

pub struct AppClock {
    last_ticks: u32,
}

impl AppClock {
    pub unsafe fn new() -> AppClock {
        AppClock {
            last_ticks: SDL_GetTicks(),
        }
    }

    pub unsafe fn step(&mut self) -> f64 {
        let now = SDL_GetTicks();
        let elapsed = (now - self.last_ticks) as f64 / 1_000.0;
        self.last_ticks = now;
        elapsed
    }
}
