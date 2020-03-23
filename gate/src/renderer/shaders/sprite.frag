#version 120

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

uniform sampler2D tex;
uniform vec2 inv_tex_dims; // inverse of tex dimensions

varying vec2 fs_inv_tex_sample_dims; // inverse width-height of sampling region, in tex pixels
varying vec2 fs_tex_vert_rb; // right-bottom vertex of sampling region, in tex pixels
varying float fs_flash_ratio;

const vec4 WHITE = vec4(1.0, 1.0, 1.0, 1.0);

void main() {
    vec2 mid = floor(fs_tex_vert_rb);
    vec2 sample_coords = mid - 0.5 + min((fs_tex_vert_rb - mid) * fs_inv_tex_sample_dims, 1.0);
    vec4 color = texture2D(tex, sample_coords * inv_tex_dims);
    gl_FragColor = mix(color, WHITE * color[3], fs_flash_ratio);
}
