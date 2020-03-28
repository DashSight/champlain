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

use crate::clutter::ClutterActor;
use crate::layer::ChamplainLayer;

#[repr(C)]
pub struct ChamplainView {
    _private: [u8; 0],
}

pub fn to_clutter_actor(input: *mut ChamplainView) -> *mut ClutterActor {
    unsafe { &mut *(input as *mut ClutterActor) }
}

/// ChamplainView functions
#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_view_new() -> *mut ChamplainView;
    fn champlain_view_center_on(view: *mut ChamplainView, latitude: c_double, longitude: c_double);
    fn champlain_view_set_zoom_level(view: *mut ChamplainView, zoom_level: c_uint);
    fn champlain_view_set_kinetic_mode(view: *mut ChamplainView, mode: bool);
    fn champlain_view_set_zoom_on_double_click(view: *mut ChamplainView, value: bool);
    fn champlain_view_add_layer(view: *mut ChamplainView, layer: *mut ChamplainLayer);
    fn champlain_view_remove_layer(view: *mut ChamplainView, layer: *mut ChamplainLayer);
}

pub fn new() -> Option<*mut ChamplainView> {
    unsafe { Some(champlain_view_new()) }
}

pub fn center_on(view: *mut ChamplainView, latitude: f64, longitude: f64) {
    unsafe { champlain_view_center_on(view, latitude, longitude) }
}

pub fn set_zoom_level(view: *mut ChamplainView, zoom_level: u32) {
    unsafe { champlain_view_set_zoom_level(view, zoom_level) }
}

pub fn set_kinetic_mode(view: *mut ChamplainView, mode: bool) {
    unsafe { champlain_view_set_kinetic_mode(view, mode) }
}

pub fn set_zoom_on_double_click(view: *mut ChamplainView, value: bool) {
    unsafe { champlain_view_set_zoom_on_double_click(view, value) }
}

pub fn add_layer(view: *mut ChamplainView, layer: *mut ChamplainLayer) {
    unsafe { champlain_view_add_layer(view, layer) }
}

pub fn remove_layer(view: *mut ChamplainView, layer: *mut ChamplainLayer) {
    unsafe { champlain_view_remove_layer(view, layer) }
}
