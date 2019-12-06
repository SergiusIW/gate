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

use std::path::Path;
use std::fs::File;
use std::io::{self, Write};
use std::ffi::OsStr;
use std::str::FromStr;

use image::{self, RgbaImage, GenericImage};
use byteorder::BigEndian;
use regex::Regex;

use crate::rect_packer::{Rect, Pack};
use crate::rerun_print;

const MAX_DIM: u32 = 512;

pub fn form_atlas(images_dir: &Path, out: &Path, pad: u32, check_rerun: bool) -> Vec<String> {
    assert!(out.extension() == None, "out must not have an extension, will use .png and .atlas extensions");
    rerun_print(check_rerun, images_dir);
    let image_out = out.with_extension("png");
    let atlas_out = out.with_extension("atlas");

    let mut images: Vec<(String, RgbaImage)> = images_dir.read_dir().expect("failed to form atlas")
        .map(|image_path| image_path.expect("failed to form atlas").path())
        .filter(|image_path| image_path.is_file() && image_path.extension() == Some(OsStr::new("png")))
        .flat_map(|image_path| {
            rerun_print(check_rerun, &image_path);
            let image = image::open(&image_path).expect("failed to form atlas");
            let name = image_path.file_stem().expect("failed to form atlas").to_str().expect("failed to form atlas");
            // TODO check name validity
            split_tiled_image(name.to_owned(), image.to_rgba())
        }).collect();

    images.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    assert!(images.windows(2).all(|w| w[0].0 != w[1].0), "should have no duplicate names");

    let atlas = Atlas::pack(images, pad).expect("failed to form atlas");
    atlas.image.save(&image_out).expect("failed to form atlas");
    rerun_print(check_rerun, &image_out);
    atlas.write_bin_to_file(&atlas_out).expect("failed to form atlas");
    rerun_print(check_rerun, &atlas_out);

    let mut regions = atlas.regions;
    let image_names = regions.drain(..).map(|(name, _)| name).collect();
    image_names
}

struct AtlasRegion {
    atlas_rect: Rect, // rect of the image in the packed atlas
    raw_sprite_rect: Rect, // rect of the trimmed image in the raw sprite coordinates, same dims as atlas_rect
    raw_sprite_dims: (u32, u32), // original height and width of the untrimmed sprite
}

impl AtlasRegion {
    fn write_bin(&self, out: &mut Vec<u8>) {
        let lt = (self.atlas_rect.pos.1 as u16, self.atlas_rect.pos.0 as u16);
        let rb = (lt.0 + self.atlas_rect.dims.1 as u16, lt.1 + self.atlas_rect.dims.0 as u16);
        let anchor_x2 = (
            2 * lt.0 as i16 + self.raw_sprite_dims.1 as i16 - 2 * self.raw_sprite_rect.pos.1 as i16,
            2 * lt.1 as i16 + self.raw_sprite_dims.0 as i16 - 2 * self.raw_sprite_rect.pos.0 as i16,
        );

        use byteorder::WriteBytesExt;
        out.write_u16::<BigEndian>(lt.0).unwrap();
        out.write_u16::<BigEndian>(lt.1).unwrap();
        out.write_u16::<BigEndian>(rb.0).unwrap();
        out.write_u16::<BigEndian>(rb.1).unwrap();
        out.write_i16::<BigEndian>(anchor_x2.0).unwrap();
        out.write_i16::<BigEndian>(anchor_x2.1).unwrap();
    }
}

fn pre_multiply_alpha(image: &mut RgbaImage) {
    for (_, _, pixel) in image.enumerate_pixels_mut() {
        let alpha = pixel[3] as f64 / 255.;
        for color_index in 0..3 {
            pixel[color_index] = (alpha * pixel[color_index] as f64).round() as u8
        }
    }
}

struct Atlas {
    regions: Vec<(String, AtlasRegion)>,
    image: RgbaImage,
}

impl Atlas {
    fn pack(mut images: Vec<(String, RgbaImage)>, pad: u32) -> Option<Atlas> {
        let trimmed_rects: Vec<_> = images.iter().map(|&(_, ref i)| trim(i)).collect();
        let image_dims: Vec<_> = trimmed_rects.iter()
            .map(|r| (r.dims.0 + 2 * pad, r.dims.1 + 2 * pad))
            .collect();
        Pack::pack(MAX_DIM, &image_dims).map(|pack| {
            let mut image = RgbaImage::new(pack.width().max(1), pack.height().max(1));
            let mut regions = Vec::new();
            for (idx, (name, sprite)) in images.drain(..).enumerate() {
                let rect = pack.rects()[idx];
                let rect = Rect {
                    pos: (rect.pos.0 + pad, rect.pos.1 + pad),
                    dims: (rect.dims.0 - 2 * pad, rect.dims.1 - 2 * pad),
                };
                let region = AtlasRegion {
                    atlas_rect: rect,
                    raw_sprite_rect: trimmed_rects[idx],
                    raw_sprite_dims: (sprite.height(), sprite.width()),
                };
                render_sprite(&mut image, &sprite, region.atlas_rect, region.raw_sprite_rect);
                regions.push((name, region));
            }
            pre_multiply_alpha(&mut image);
            Atlas { regions, image }
        })
    }

    fn write_bin(&self) -> Vec<u8> {
        use byteorder::WriteBytesExt;
        let mut out = Vec::new();
        out.write_u16::<BigEndian>(self.image.width() as u16).unwrap();
        out.write_u16::<BigEndian>(self.image.height() as u16).unwrap();
        out.write_u16::<BigEndian>(self.regions.len() as u16).unwrap();
        for &(_, ref region) in self.regions.iter() {
            region.write_bin(&mut out);
        }
        out
    }

    fn write_bin_to_file(&self, path: &Path) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.write_bin())?;
        Ok(())
    }
}

lazy_static! {
    static ref TILED_REGEX: Regex = Regex::new("(.*)_t([0-9]+)").unwrap();
}

fn split_tiled_image(name: String, mut image: RgbaImage) -> Vec<(String, RgbaImage)> {
    let result = TILED_REGEX.captures(&name).map(|caps| {
        let tile_width = u32::from_str(&caps[2]).expect("invalid tile width");
        assert!(tile_width > 0, "tile width must be positive");
        assert!(image.width() % tile_width == 0 && image.height() % tile_width == 0,
                "image dimensions are not divisible by tile width");
        let mut result = Vec::new();
        for row in 0..(image.height() / tile_width) {
            for col in 0..(image.width() / tile_width) {
                let sub_image = image.sub_image(col * tile_width, row * tile_width, tile_width, tile_width);
                if sub_image.pixels().any(|(_, _, p)| p[3] != 0) {
                    let sub_image = sub_image.to_image();
                    let name = format!("{}R{}C{}", &caps[1], row, col);
                    result.push((name, sub_image));
                }
            }
        }
        result
    });
    result.unwrap_or(vec![(name, image)])
}

fn render_sprite(atlas: &mut RgbaImage, sprite: &RgbaImage, dst_rect: Rect, src_rect: Rect) {
    assert!(dst_rect.dims == src_rect.dims);
    for row in 0..dst_rect.dims.0 {
        for col in 0..dst_rect.dims.1 {
            let out_color = *sprite.get_pixel(src_rect.pos.1 + col, src_rect.pos.0 + row);
            *atlas.get_pixel_mut(dst_rect.pos.1 + col, dst_rect.pos.0 + row) = out_color;
        }
    }
}

fn trim(image: &RgbaImage) -> Rect {
    let rows = 0..image.height();
    let cols = 0..image.width();
    let row_has_pixel = |&row: &u32| cols.clone().any(|col| image.get_pixel(col, row)[3] != 0);
    let col_has_pixel = |&col: &u32| rows.clone().any(|row| image.get_pixel(col, row)[3] != 0);

    let top = rows.clone().find(&row_has_pixel).expect("image contains no pixels with non-zero alpha");
    let left = cols.clone().find(&col_has_pixel).unwrap();
    let bottom = rows.clone().rev().find(&row_has_pixel).unwrap() + 1;
    let right = cols.clone().rev().find(&col_has_pixel).unwrap() + 1;

    Rect { pos: (top, left), dims: (bottom - top, right - left) }
}
