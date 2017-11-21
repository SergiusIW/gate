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

use std::io::{self, BufReader};
use std::fs::File;
use std::collections::HashMap;

use byteorder::BigEndian;

use sdl2::render::Texture;

pub struct Atlas {
    pub tex: Texture,
    pub dims: (f32, f32),
    pub images: HashMap<u16, ImageCoords>,
}

impl Atlas {
    pub fn new(tex: Texture, filename: &str, pad: u16) -> io::Result<Atlas> {
        use byteorder::ReadBytesExt;

        let mut file = BufReader::new(File::open(filename)?);

        let dims = (file.read_u16::<BigEndian>()? as f32, file.read_u16::<BigEndian>()? as f32);
        let handle_count = file.read_u16::<BigEndian>()?;

        let mut images = HashMap::with_capacity(handle_count as usize);
        for id in 0..handle_count {
            let image = ImageCoords {
                lt: ((file.read_u16::<BigEndian>()? - pad) as f32, (file.read_u16::<BigEndian>()? - pad) as f32),
                rb: ((file.read_u16::<BigEndian>()? + pad) as f32, (file.read_u16::<BigEndian>()? + pad) as f32),
                anchor: (0.5 * file.read_i16::<BigEndian>()? as f32, 0.5 * file.read_i16::<BigEndian>()? as f32),
            };
            images.insert(id, image);
        }

        Ok(Atlas { tex, dims, images })
    }
}

// note: all ImageCoords are coordinates in pixels relative to top-left origin
#[derive(Copy, Clone)]
pub struct ImageCoords {
    pub lt: (f32, f32), // left, top
    pub rb: (f32, f32), // right, bottom
    pub anchor: (f32, f32), // anchor X, Y
}
