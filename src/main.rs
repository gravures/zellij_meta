// Copyright (c) 2024 - Gilles Coissac
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>
use std::collections::BTreeMap;

use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    config: BTreeMap<String, String>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, config: BTreeMap<String, String>) {
        self.config = config;
    }

    fn update(&mut self, _event: Event) -> bool {
        return false;
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        println!("");
        println!("Hello Zellijer!");
    }
}
