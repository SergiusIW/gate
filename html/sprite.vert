#version 300 es

// Copyright 2017 Matthew D. Michelotti
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

// FIXME avoid duplicating shader code in the codebase

in vec2 vert;
in vec2 vs_tex_vert_lt; // pass in vs_tex_vert - 0.5 / scale_xy
in vec2 vs_tex_vert_rb; // pass in vs_tex_vert + 0.5 / scale_xy
in float vs_flash_ratio;

out vec2 fs_tex_vert_lt;
out vec2 fs_tex_vert_rb;
out float fs_flash_ratio;

void main() {
    fs_tex_vert_lt = vs_tex_vert_lt;
    fs_tex_vert_rb = vs_tex_vert_rb;
    fs_flash_ratio = vs_flash_ratio;
    gl_Position = vec4(vert, 0, 1);
}
