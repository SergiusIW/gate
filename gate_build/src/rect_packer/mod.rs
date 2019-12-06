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

mod bit_grid;

use std::u32;

use self::bit_grid::BitGrid;

const MAX_MAX_DIM: u32 = 10_000;

#[derive(Copy, Clone)]
pub struct Rect {
    pub pos: (u32, u32), // upper-left (row, col)
    pub dims: (u32, u32), // (height, width)
}

pub struct Pack {
    rects: Vec<Rect>,
    dims: (u32, u32), // (height, width), must be power of 2
}

impl Pack {
    pub fn height(&self) -> u32 { self.dims.0 } // will be power of 2
    pub fn width(&self) -> u32 { self.dims.1 } // will be power of 2
    pub fn rects(&self) -> &[Rect] { &self.rects }
    pub fn area(&self) -> u32 { self.height() * self.width() }

    // max_dim: maximum allowed height or width, must be power of 2
    // rects: rectangle dimensions (height, width) to pack
    pub fn pack(max_dim: u32, rects: &[(u32, u32)]) -> Option<Pack> {
        assert!(max_dim <= MAX_MAX_DIM && is_pow_2(max_dim));
        assert!(rects.iter().all(|r| 0 < r.0 && 0 < r.1));
        if rects.iter().any(|r| r.0 > max_dim || r.1 > max_dim) {
            return None;
        }

        let mut order_and_rects: Vec<_> = rects.iter().cloned().enumerate().collect();
        order_and_rects.sort_by_key(|&(_, (h, w))| u32::MAX - h * w);
        let order: Vec<_> = order_and_rects.iter().map(|&(idx, _)| idx).collect();
        let rects: Vec<_> = order_and_rects.iter().map(|&(_, dims)| dims).collect();

        (0..(log_ceil_pow_2(max_dim) + 1))
            .map(|lw| 1 << lw)
            .filter_map(|w| try_pack_rects(max_dim, w, &rects))
            .min_by_key(|pack| pack.area())
            .map(|mut pack| {
                permute(&mut pack.rects, &order);
                pack
            })
    }
}

fn permute(vals: &mut [Rect], order: &[usize]) {
    assert!(vals.len() == order.len());
    let mut result = vec![None; vals.len()];
    for idx in 0..vals.len() {
        result[order[idx]] = Some(vals[idx]);
    }
    for idx in 0..vals.len() {
        vals[idx] = result[idx].unwrap();
    }
}

fn log_ceil_pow_2(value: u32) -> u32 {
    (0..32).find(|&log| 1 << log >= value).expect("value is too large")
}

fn ceil_pow_2(value: u32) -> u32 {
    1 << log_ceil_pow_2(value)
}

fn is_pow_2(value: u32) -> bool {
    ceil_pow_2(value) == value
}

fn overall_dims(rects: &[Rect]) -> (u32, u32) {
    (
        rects.iter().map(|r| ceil_pow_2(r.pos.0 + r.dims.0)).max().unwrap_or(0),
        rects.iter().map(|r| ceil_pow_2(r.pos.1 + r.dims.1)).max().unwrap_or(0),
    )
}

fn try_pack_rects(max_height: u32, width: u32, ordered_rects: &[(u32, u32)]) -> Option<Pack> {
    let mut area = 0;
    for &(rect_height, rect_width) in ordered_rects.iter() {
        area += (rect_height) * (rect_width);
        if rect_height > max_height || rect_width > width {
            return None;
        }
    }
    if area > max_height * width {
        return None;
    }

    let mut grid = BitGrid::new(max_height, width);
    let mut result = Vec::with_capacity(ordered_rects.len());
    for &(rect_height, rect_width) in ordered_rects.iter() {
        if let Some(pos) = grid.fill_rect(rect_height, rect_width) {
            let pos = (pos.0, pos.1);
            result.push(Rect { pos, dims: (rect_height, rect_width) });
        } else {
            return None;
        }
    }
    Some(Pack { dims: overall_dims(&result), rects: result })
}
