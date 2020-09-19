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

use std::io::{self, Read};
use std::collections::HashMap;

use byteorder::BigEndian;

const PAD: u16 = 1;

pub struct Atlas {
    #[allow(dead_code)] // dims might not used when targeting wasm
    pub(super) dims: (f32, f32),
    pub(super) images: HashMap<u16, ImageCoords>,
}

impl Atlas {
    pub fn new<R: Read>(mut input: R) -> io::Result<Atlas> {
        use byteorder::ReadBytesExt;

        let dims = (input.read_u16::<BigEndian>()? as f32, input.read_u16::<BigEndian>()? as f32);
        let handle_count = input.read_u16::<BigEndian>()?;

        let mut images = HashMap::with_capacity(handle_count as usize);
        for id in 0..handle_count {
            let image = ImageCoords {
                lt: ((input.read_u16::<BigEndian>()? - PAD) as f32, (input.read_u16::<BigEndian>()? - PAD) as f32),
                rb: ((input.read_u16::<BigEndian>()? + PAD) as f32, (input.read_u16::<BigEndian>()? + PAD) as f32),
                anchor: (0.5 * input.read_i16::<BigEndian>()? as f32, 0.5 * input.read_i16::<BigEndian>()? as f32),
            };
            images.insert(id, image);
        }

        Ok(Atlas { dims, images })
    }
}

// note: all ImageCoords are coordinates in pixels relative to top-left origin
#[derive(Copy, Clone)]
pub(super) struct ImageCoords {
    pub lt: (f32, f32), // left, top
    pub rb: (f32, f32), // right, bottom
    pub anchor: (f32, f32), // anchor X, Y
}
