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

use libc::{c_char, c_int};
use std::ptr;

use crate::marker::ChamplainMarker;

#[repr(C)]
#[derive(Clone)]
pub struct ClutterActor {
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
    fn clutter_stage_new() -> *mut ClutterActor;
}

pub fn init() -> Error {
    unsafe { clutter_init(ptr::null(), ptr::null()) }
}

pub fn stage_new() -> *mut ClutterActor {
    unsafe { clutter_stage_new() }
}
