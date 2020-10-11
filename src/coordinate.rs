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

use crate::clutter::{ClutterActor, ClutterActorSys};
use crate::location::{ChamplainLocation, ChamplainLocationSys};

#[repr(C)]
pub struct ChamplainCoordinateSys {
    _private: [u8; 0],
}

#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_coordinate_new() -> *mut ChamplainCoordinateSys;
    fn champlain_coordinate_new_full(latitude: f64, longitude: f64) -> *mut ChamplainCoordinateSys;
}

pub struct ChamplainCoordinate {
    actor: ClutterActor,
    location: ChamplainLocation,
}

impl ChamplainCoordinate {
    pub(crate) fn new_with_ptr(ptr: *mut ChamplainCoordinateSys) -> Self {
        Self {
            actor: ClutterActor::new(ptr as *mut ClutterActorSys),
            location: ChamplainLocation::new_with_ptr(ptr as *mut ChamplainLocationSys),
        }
    }

    pub fn borrow_mut_actor(&mut self) -> &mut ClutterActor {
        &mut self.actor
    }

    pub fn borrow_mut_location(&mut self) -> &mut ChamplainLocation {
        &mut self.location
    }

    pub fn new() -> Self {
        unsafe { Self::new_with_ptr(champlain_coordinate_new()) }
    }

    pub fn new_full(latitude: f64, longitude: f64) -> Self {
        unsafe { Self::new_with_ptr(champlain_coordinate_new_full(latitude, longitude)) }
    }
}
