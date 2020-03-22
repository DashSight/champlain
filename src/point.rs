/*
 * Copyright 2020 Alistair Francis <alistair@alistair23.me>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::clutter::ClutterActor;
use crate::clutter_colour::ClutterColor;

#[repr(C)]
pub struct ChamplainPoint {
    _private: [u8; 0],
}

/// ChamplainMarker functions
#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_point_new() -> *mut ClutterActor;
    fn champlain_point_new_full(size: f64, colour: *const ClutterColor) -> *mut ClutterActor;
    fn champlain_point_set_color(point: *mut ChamplainPoint, colour: *const ClutterColor);
}

pub fn new() -> *mut ClutterActor {
    unsafe { champlain_point_new() }
}

pub fn new_full(size: f64, colour: *const ClutterColor) -> *mut ClutterActor {
    unsafe { champlain_point_new_full(size, colour) }
}

pub fn set_colour(point: *mut ChamplainPoint, colour: *const ClutterColor) {
    unsafe { champlain_point_set_color(point, colour) }
}
