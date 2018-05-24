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

extern crate gate;

use gate::{App, AppContext, AppInfo, KeyCode};
use gate::renderer::{Renderer, Affine};

mod asset_id { include!(concat!(env!("OUT_DIR"), "/asset_id.rs")); }
use asset_id::{AssetId, SpriteId, MusicId, SoundId};

// Note: the assets that we placed in the src_assets directory can be referenced using the
//       SpriteId, MusicId, and SoundId enums

struct HeldDisc { value: u8, pos: (f64, f64) }

fn disc_sprite(value: u8) -> SpriteId {
    match value {
        0 => SpriteId::Disc0,
        1 => SpriteId::Disc1,
        2 => SpriteId::Disc2,
        3 => SpriteId::Disc3,
        4 => SpriteId::Disc4,
        _ => panic!("illegal disc value {}", value),
    }
}

fn disc_pos(pillar_index: usize, height_index: usize) -> (f64, f64) {
    (-27. + pillar_index as f64 * 27., 2.5 + height_index as f64 * 5.)
}

fn pillar_for_cursor(cursor: (f64, f64), dims: (f64, f64)) -> Option<usize> {
    let cursor = (cursor.0 - 0.5 * dims.0, cursor.1 - 0.5 * dims.1);
    (0..3).find(|&idx| {
        let cursor_x = cursor.0 + 13.5 - (13.5 * idx as f64);
        cursor_x > -6. && cursor_x < 6. && cursor.1 > -5.5 && cursor.1 < 9.
    })
}

struct TowerGame { pillars: Vec<Vec<u8>>, held: Option<HeldDisc> }

impl App<AssetId> for TowerGame {
    fn start(&mut self, ctx: &mut AppContext<AssetId>) {
        ctx.audio.loop_music(MusicId::Tick);
    }

    fn advance(&mut self, seconds: f64, _ctx: &mut AppContext<AssetId>) {
        if let Some(held) = self.held.as_mut() {
            held.pos.1 = (held.pos.1 + seconds * 200.).min(35.);
        }
    }

    fn key_down(&mut self, key: KeyCode, ctx: &mut AppContext<AssetId>) {
        let index = match key {
            KeyCode::Num1 => Some(0),
            KeyCode::Num2 => Some(1),
            KeyCode::Num3 => Some(2),
            KeyCode::MouseLeft => pillar_for_cursor(ctx.cursor(), ctx.dims()),
            _ => None,
        };
        if let Some(index) = index {
            let pillar = &mut self.pillars[index];
            if let Some(held) = self.held.take() {
                if pillar.last().map_or(true, |&v| v > held.value) {
                    pillar.push(held.value);
                    ctx.audio.play_sound(SoundId::Shuffle);
                } else {
                    self.held = Some(held);
                    ctx.audio.play_sound(SoundId::Error);
                }
            } else {
                if let Some(value) = pillar.pop() {
                    let pos = disc_pos(index, pillar.len());
                    self.held = Some(HeldDisc { value, pos });
                    ctx.audio.play_sound(SoundId::Shuffle);
                } else {
                    ctx.audio.play_sound(SoundId::Error);
                }
            }
        }
    }

    fn render(&mut self, renderer: &mut Renderer<AssetId>, ctx: &AppContext<AssetId>) {
        let (app_width, app_height) = ctx.dims();
        let mut renderer = renderer.sprite_mode();
        for x in 0..((app_width / 16.).ceil() as usize) {
            for y in 0..((app_height / 16.).ceil() as usize) {
                let affine = Affine::translate(8. + x as f64 * 16., 8. + y as f64 * 16.);
                let tile = if (x + y) % 2 == 0 { SpriteId::BgTileR0C0 } else { SpriteId::BgTileR0C1 };
                renderer.draw(&affine, tile);
            }
        }
        let base = Affine::translate(0.5 * app_width, 0.5 * app_height - 5.).pre_scale(0.5);
        renderer.draw(&base, SpriteId::Pillars);
        for pillar_index in 0..self.pillars.len() {
            let pillar = &self.pillars[pillar_index];
            for height_index in 0..pillar.len() {
                let pos = disc_pos(pillar_index, height_index);
                renderer.draw(&base.pre_translate(pos.0, pos.1), disc_sprite(pillar[height_index]));
            }
        }
        if let Some(held) = self.held.as_ref() {
            renderer.draw(&base.pre_translate(held.pos.0, held.pos.1), disc_sprite(held.value));
        }
    }
}

fn main() {
    let info = AppInfo::with_max_dims(86., 48.)
                       .min_dims(64., 44.)
                       .tile_width(16)
                       .title("Tower");
    gate::run(info, TowerGame { pillars: vec![vec![4, 3, 2, 1, 0], vec![], vec![]], held: None });
}
