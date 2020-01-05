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

use libc::{c_double, c_uint};

#[repr(C)] pub struct ChamplainView { _private: [u8; 0] }

/// ChamplainView functions
#[link(name = "champlain-0.12")]
extern {
    fn champlain_view_new () -> *mut ChamplainView;
    fn champlain_view_center_on (view: *mut ChamplainView,
                                 latitude: c_double,
                                 longitude: c_double);
    fn champlain_view_set_zoom_level (view: *mut ChamplainView,
                                      zoom_level: c_uint);
}

pub fn view_new() -> Option<*mut ChamplainView> {
    unsafe {
        Some(champlain_view_new())
    }
}

pub fn view_center_on(view: *mut ChamplainView,
                      latitude: f64,
                      longitude: f64) {
    unsafe {
        champlain_view_center_on(view, latitude, longitude)
    }
}

pub fn view_set_zoom_level(view: *mut ChamplainView,
                           zoom_level: u32) {
    unsafe {
        champlain_view_set_zoom_level(view, zoom_level)
    }
}
