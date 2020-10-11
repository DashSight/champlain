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

use crate::clutter::ClutterActorSys;
use crate::clutter_colour::{ClutterColor, ClutterColorSys};
use crate::location::{ChamplainLocation, ChamplainLocationSys};
use crate::marker::{ChamplainMarker, ChamplainMarkerSys};

#[repr(C)]
pub(crate) struct ChamplainPointSys {
    _private: [u8; 0],
}

/// ChamplainMarker functions
#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_point_new() -> *mut ClutterActorSys;
    fn champlain_point_new_full(size: f64, colour: *const ClutterColorSys) -> *mut ClutterActorSys;
    fn champlain_point_set_color(point: *mut ChamplainPointSys, colour: *const ClutterColorSys);
    fn champlain_point_get_color(point: *mut ChamplainPointSys) -> *const ClutterColorSys;
}

pub struct ChamplainPoint {
    ptr: *mut ChamplainPointSys,
    location: ChamplainLocation,
    marker: ChamplainMarker,
}

impl ChamplainPoint {
    pub(crate) fn new_with_ptr(ptr: *mut ChamplainPointSys) -> Self {
        Self {
            ptr,
            location: ChamplainLocation::new_with_ptr(ptr as *mut ChamplainLocationSys),
            marker: ChamplainMarker::new_with_ptr(ptr as *mut ChamplainMarkerSys),
        }
    }

    fn get_ptr(&mut self) -> *mut ChamplainPointSys {
        self.ptr
    }

    pub fn new() -> Self {
        unsafe { Self::new_with_ptr(champlain_point_new() as *mut ChamplainPointSys) }
    }

    pub fn new_full(size: f64, colour: ClutterColor) -> Self {
        unsafe {
            Self::new_with_ptr(
                champlain_point_new_full(size, colour.get_ptr()) as *mut ChamplainPointSys
            )
        }
    }

    pub fn set_colour(&mut self, colour: ClutterColor) {
        unsafe { champlain_point_set_color(self.get_ptr(), colour.get_ptr()) }
    }

    pub fn get_colour(&mut self) -> ClutterColor {
        unsafe { ClutterColor::new_with_ptr(champlain_point_get_color(self.get_ptr())) }
    }

    pub fn set_location(&mut self, lat: f64, lon: f64) {
        let mut location =
            ChamplainLocation::new_with_ptr(self.get_ptr() as *mut ChamplainLocationSys);
        location.set_location(lat, lon)
    }

    pub fn borrow_mut_location(&mut self) -> &mut ChamplainLocation {
        &mut self.location
    }

    pub fn borrow_mut_marker(&mut self) -> &mut ChamplainMarker {
        &mut self.marker
    }
}
