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

use crate::layer::ChamplainLayer;
use crate::location::ChamplainLocation;
use crate::marker::ChamplainMarker;
use crate::point::{ChamplainPoint, ChamplainPointSys};
use libc::{c_char, c_int};
use std::ptr;

pub struct ClutterActor {
    ptr: *mut ClutterActorSys,
}

impl ClutterActor {
    pub(crate) fn new(ptr: *mut ClutterActorSys) -> Self {
        Self { ptr }
    }

    pub(crate) fn get_ptr(&self) -> *mut ClutterActorSys {
        self.ptr
    }

    // TODO: This is unsafe as now we have two copied of self.get_ptr()
    pub fn to_point(&self) -> ChamplainPoint {
        unsafe { ChamplainPoint::new(&mut *(self.get_ptr() as *mut ChamplainPointSys)) }
    }

    // TODO: This is unsafe as now we have two copied of self.get_ptr()
    pub fn to_champlain_layer(&self) -> *mut ChamplainLayer {
        unsafe { &mut *(self.get_ptr() as *mut ChamplainLayer) }
    }

    // TODO: This is unsafe as now we have two copied of self.get_ptr()
    pub fn to_champlain_marker(&self) -> *mut ChamplainMarker {
        unsafe { &mut *(self.get_ptr() as *mut ChamplainMarker) }
    }

    // TODO: This is unsafe as now we have two copied of self.get_ptr()
    pub fn to_location(&self) -> *mut ChamplainLocation {
        unsafe { &mut *(self.get_ptr() as *mut ChamplainLocation) }
    }
}

#[repr(C)]
pub struct ClutterActorSys {
    _private: [u8; 0],
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum Error {
    CLUTTER_INIT_SUCCESS = 1,
    CLUTTER_INIT_ERROR_UNKNOWN = 0,
    CLUTTER_INIT_ERROR_THREADS = -1,
    CLUTTER_INIT_ERROR_BACKEND = -2,
    CLUTTER_INIT_ERROR_INTERNAL = -3,
}

#[link(name = "clutter-1.0")]
extern "C" {
    fn clutter_init(argc: *const c_int, argv: *const *const c_char) -> Error;
    fn clutter_stage_new() -> *mut ClutterActorSys;
}

pub fn init() -> Error {
    unsafe { clutter_init(ptr::null(), ptr::null()) }
}

pub fn stage_new() -> ClutterActor {
    unsafe { ClutterActor::new(clutter_stage_new()) }
}
