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

pub struct BitGrid {
    bits: Vec<bool>,
    height: u32,
    width: u32,
}

impl BitGrid {
    pub fn new(height: u32, width: u32) -> BitGrid {
        BitGrid {
            bits: vec![false; (height * width) as usize],
            height,
            width,
        }
    }

    // returns (row, col) of filled rectangle, or None if there is no room for it
    pub fn fill_rect(&mut self, height: u32, width: u32) -> Option<(u32, u32)> {
        for row in 0..(self.height - height + 1) {
            for col in 0..(self.width - width + 1) {
                if self.fill_rect_at(height, width, row, col) {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn fill_rect_at(&mut self, height: u32, width: u32, row: u32, col: u32) -> bool {
        for row in row..(row + height) {
            for col in col..(col + width) {
                if self.is_set(row, col) {
                    return false;
                }
            }
        }
        for row in row..(row + height) {
            for col in col..(col + width) {
                self.set(row, col);
            }
        }
        true
    }

    fn idx(&self, row: u32, col: u32) -> usize {
        assert!(col < self.width);
        (row * self.width + col) as usize
    }

    fn is_set(&self, row: u32, col: u32) -> bool {
        self.bits[self.idx(row, col)]
    }

    fn set(&mut self, row: u32, col: u32) {
        let idx = self.idx(row, col);
        self.bits[idx] = true;
    }
}
