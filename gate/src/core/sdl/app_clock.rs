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

use sdl2_sys as sdl;

use crate::app_info::AppInfo;
use std::u32;

pub struct AppClock {
    last_ticks: u32,
    frame_dur_millis: u32,
    work_load_sum: f64,
    work_load_max: f64,
    work_load_frames: u64,
    last_print_workload_ticks: u32,
}

impl AppClock {
    pub fn new(info: &AppInfo) -> AppClock {
        AppClock {
            last_ticks: sdl::SDL_GetTicks(),
            frame_dur_millis: (1000. / info.target_fps).round() as u32,
            work_load_sum: 0.0,
            work_load_max: 0.0,
            work_load_frames: 0,
            last_print_workload_ticks: if info.print_workload_info { 1 } else { u32::MAX - 100_000 },
        }
    }

    pub fn step(&mut self) -> f64 {
        let mut elapsed;
        let mut first_iter = true;
        loop {
            let now = sdl::SDL_GetTicks();
            let dt = now - self.last_ticks;
            elapsed = dt as f64 / 1_000.0;

            if first_iter {
                first_iter = false;
                self.append_workload(now, dt);
            }

            if dt < self.frame_dur_millis {
                sdl::SDL_Delay(self.frame_dur_millis - dt);
            } else {
                break;
            }
        }
        self.last_ticks = sdl::SDL_GetTicks();
        elapsed
    }

    fn append_workload(&mut self, now: u32, dt: u32) {
        let work_load = dt as f64 / self.frame_dur_millis as f64;
        self.work_load_sum += work_load;
        self.work_load_max = self.work_load_max.max(work_load);
        self.work_load_frames += 1;
        if now > self.last_print_workload_ticks + 3_000 {
            println!("Work Load: Average {:.1}%, Max {:.1}%", 100.0 * self.work_load_sum / self.work_load_frames as f64, 100.0 * self.work_load_max);
            self.work_load_sum = 0.0;
            self.work_load_max = 0.0;
            self.work_load_frames = 0;
            self.last_print_workload_ticks = now;
        }
    }
}
