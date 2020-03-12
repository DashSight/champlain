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

use crate::location::ChamplainLocation;

#[repr(C)]
pub struct ChamplainCoordinate {
    _private: [u8; 0],
}

pub fn to_location(input: *mut ChamplainCoordinate) -> *mut ChamplainLocation {
    unsafe { &mut *(input as *mut ChamplainLocation) }
}

#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_coordinate_new() -> *mut ChamplainCoordinate;
    fn champlain_coordinate_new_full(latitude: f64, longitude: f64) -> *mut ChamplainCoordinate;
}

pub fn new() -> *mut ChamplainCoordinate {
    unsafe { champlain_coordinate_new() }
}

pub fn new_full(latitude: f64, longitude: f64) -> *mut ChamplainCoordinate {
    unsafe { champlain_coordinate_new_full(latitude, longitude) }
}
