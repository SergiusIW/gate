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

use gate::{App, Audio};
use gate::app_info::AppInfo;
use gate::input::{InputEvent, KeyCode, MouseButton};
use gate::renderer::{Renderer, Affine};

mod asset_id { include!(concat!(env!("OUT_DIR"), "/asset_id.rs")); }
use asset_id::{AssetId, SpriteId, TileId, MusicId, SoundId};

// Note: the assets that we placed in the src_assets directory can be referenced using the
//       SpriteId, TileId, MusicId, and SoundId enums

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

struct TowerGame { pillars: Vec<Vec<u8>>, held: Option<HeldDisc> }

impl App<AssetId> for TowerGame {
    fn start(&mut self, audio: &mut Audio<AssetId>) {
        audio.loop_music(MusicId::Tick);
    }

    fn advance(&mut self, seconds: f64, _audio: &mut Audio<AssetId>) -> bool {
        if let Some(held) = self.held.as_mut() {
            held.pos.1 = (held.pos.1 + seconds * 200.).min(35.);
        }
        true // continue the game
    }

    fn input(&mut self, evt: InputEvent, audio: &mut Audio<AssetId>) -> bool {
        let index = match evt {
            InputEvent::KeyPressed(KeyCode::Num1) => Some(0),
            InputEvent::KeyPressed(KeyCode::Num2) => Some(1),
            InputEvent::KeyPressed(KeyCode::Num3) => Some(2),
            InputEvent::MouseReleased(MouseButton::Left, x, _) if x > -21. && x <= -7. => Some(0),
            InputEvent::MouseReleased(MouseButton::Left, x, _) if x > -7. && x < 7. => Some(1),
            InputEvent::MouseReleased(MouseButton::Left, x, _) if x >= 7. && x < 21. => Some(2),
            _ => None,
        };

        if let Some(index) = index {
            let pillar = &mut self.pillars[index];
            if let Some(held) = self.held.take() {
                if pillar.last().map_or(true, |&v| v > held.value) {
                    pillar.push(held.value);
                    audio.play_sound(SoundId::Shuffle);
                } else {
                    self.held = Some(held);
                    audio.play_sound(SoundId::Error);
                }
            } else {
                if let Some(value) = pillar.pop() {
                    let pos = disc_pos(index, pillar.len());
                    self.held = Some(HeldDisc { value, pos });
                    audio.play_sound(SoundId::Shuffle);
                } else {
                    audio.play_sound(SoundId::Error);
                }
            }
        }
        true // continue the game
    }

    fn render(&mut self, renderer: &mut Renderer<AssetId>) {
        let (app_width, app_height) = (renderer.app_width(), renderer.app_height());
        { // drawing tiles
            let mut renderer = renderer.tiled_mode(0.5 * app_width, 0.5 * app_height);
            for x in 0..((app_width / 16.).ceil() as usize) {
                for y in 0..((app_height / 16.).ceil() as usize) {
                    let affine = Affine::translate(8. + x as f64 * 16., 8. + y as f64 * 16.);
                    let tile = if (x + y) % 2 == 0 { TileId::BgTileR0C0 } else { TileId::BgTileR0C1 };
                    renderer.draw(&affine, tile);
                }
            }
        }
        { // drawing sprites
            let mut renderer = renderer.sprite_mode();
            let base = Affine::scale(0.5).pre_translate(0., -10.);
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
}

fn main() {
    let info = AppInfo::with_app_height(48.).title("Tower").build();
    gate::run(info, TowerGame { pillars: vec![vec![4, 3, 2, 1, 0], vec![], vec![]], held: None });
}
