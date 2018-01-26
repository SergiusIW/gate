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
uniform vec2 inv_tex_dims; // inverse of tex dimensions
uniform float inv_tex_sample_dim; // inverse width-height of sampling region, in tex pixels

varying vec2 fs_tex_vert_rb; // right-bottom vertex of sampling region, in tex pixels

void main() {
    vec2 mid = floor(fs_tex_vert_rb);
    vec2 sample_coords = mid - 0.5 + min((fs_tex_vert_rb - mid) * inv_tex_sample_dim, 1.0);
    gl_FragColor = texture2D(tex, sample_coords * inv_tex_dims);
}
