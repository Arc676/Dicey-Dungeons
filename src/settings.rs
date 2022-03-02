// MIT/Apache 2.0 dual license
// Apache 2.0
// Copyright 2022 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// MIT
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::AppState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::{Separator, Slider, Ui};

pub struct GameSettings {
    players: u32,
    map_width: u32,
    map_height: u32,
    initial_travel_distance: u32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            players: 2,
            map_width: 10,
            map_height: 10,
            initial_travel_distance: 5,
        }
    }
}

impl GameSettings {
    pub fn reset_settings(&mut self) {
        *self = GameSettings::default();
    }
}

fn number_setting(ui: &mut Ui, num: &mut u32, min: u32, max: u32, lbl: &str) {
    ui.label(lbl);
    let slider = Slider::new(num, min..=max);
    ui.add(slider);
}

pub fn settings_ui(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    mut settings: ResMut<GameSettings>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.heading("Dicey Dungeons: Settings");

        number_setting(ui, &mut settings.players, 2, 6, "Number of players");
        number_setting(ui, &mut settings.map_width, 5, 20, "Map width");
        number_setting(ui, &mut settings.map_height, 5, 20, "Map height");

        let sep = Separator::default().spacing(12.).horizontal();
        ui.add(sep);

        ui.label("All players' starting positions will be connected to the goal by a path of \
         a fixed length before additional paths are generated. This initial distance can be \
         freely chosen.");
        number_setting(ui, &mut settings.initial_travel_distance, 2, 20, "Initial travel distance");

        let sep = Separator::default().spacing(12.).horizontal();
        ui.add(sep);

        if ui.button("Revert to default settings").clicked() {
            settings.reset_settings();
        }

        let sep = Separator::default().spacing(12.).horizontal();
        ui.add(sep);

        if ui.button("Back to Main").clicked() {
            state.set(AppState::MainMenu).unwrap();
        }
    });
}
