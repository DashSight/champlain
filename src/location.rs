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

use libc::c_double;

#[repr(C)]
pub struct ChamplainLocationSys {
    _private: [u8; 0],
}

/// ChamplainMarker functions
#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_location_set_location(
        location: *mut ChamplainLocationSys,
        lat: c_double,
        lon: c_double,
    );
}

pub struct ChamplainLocation {
    ptr: *mut ChamplainLocationSys,
}

impl ChamplainLocation {
    pub(crate) fn new_with_ptr(ptr: *mut ChamplainLocationSys) -> Self {
        Self { ptr }
    }

    pub(crate) fn get_ptr(&mut self) -> *mut ChamplainLocationSys {
        self.ptr
    }

    pub fn set_location(&mut self, lat: f64, lon: f64) {
        unsafe { champlain_location_set_location(self.get_ptr(), lat, lon) }
    }
}
