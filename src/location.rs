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
use libc::c_double;

#[repr(C)]
pub struct ChamplainLocation {
    _private: [u8; 0],
}

pub fn to_location(actor: *mut ClutterActor) -> *mut ChamplainLocation {
    unsafe { std::mem::transmute::<*mut ClutterActor, *mut ChamplainLocation>(actor) }
}

/// ChamplainMarker functions
#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_location_set_location(
        location: *mut ChamplainLocation,
        lat: c_double,
        lon: c_double,
    );
}

pub fn set_location(location: *mut ChamplainLocation, lat: f64, lon: f64) {
    unsafe { champlain_location_set_location(location, lat, lon) }
}
