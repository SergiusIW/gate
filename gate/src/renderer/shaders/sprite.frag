#version 100

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

precision highp float;

uniform sampler2D tex;
uniform vec2 tex_dims;

varying vec2 fs_tex_vert_lt; // left-top vertex of sampling region
varying vec2 fs_tex_vert_rb; // right-bottom vertex of sampling region
varying float fs_flash_ratio;

const vec3 WHITE = vec3(1.0, 1.0, 1.0);

void main() {
    vec2 low = fs_tex_vert_lt * tex_dims; // TODO do this ahead of time, not in shader
    vec2 high = fs_tex_vert_rb * tex_dims; // TODO do this ahead of time, not in shader
    vec2 mid = floor(high);
    vec2 sample_coords = mid + 0.5 - max((mid - low) / (high - low), 0.0);
    vec4 color = texture2D(tex, sample_coords / tex_dims);
    gl_FragColor = vec4(mix(vec3(color[0], color[1], color[2]), WHITE * color[3], fs_flash_ratio), color[3]);
}
