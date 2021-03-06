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

use std::ffi::CStr;
use std::os::raw::c_int;
use super::sdl_imports::*;

// TODO make sure to call this on all relevant sdl return values
pub trait SdlErrorCode {
    type Out;
    unsafe fn sdl_check(self) -> Self::Out;
}

impl SdlErrorCode for c_int {
    type Out = ();
    unsafe fn sdl_check(self) {
        sdl_assert(self == 0);
    }
}

impl<T> SdlErrorCode for *mut T {
    type Out = Self;
    unsafe fn sdl_check(self) -> Self {
        sdl_assert(!self.is_null());
        self
    }
}

pub unsafe fn sdl_assert(condition: bool) {
    if !condition {
        let message = SDL_GetError();
        panic!("SDL error: {}", CStr::from_ptr(message).to_str().unwrap());
    }
}
