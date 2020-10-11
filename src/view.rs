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
use crate::layer::{ChamplainLayer, ChamplainLayerSys};
use libc::{c_double, c_uint};

#[repr(C)]
pub(crate) struct ChamplainViewSys {
    _private: [u8; 0],
}

/// ChamplainViewSys functions
#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_view_center_on(
        view: *mut ChamplainViewSys,
        latitude: c_double,
        longitude: c_double,
    );
    fn champlain_view_set_zoom_level(view: *mut ChamplainViewSys, zoom_level: c_uint);
    fn champlain_view_set_kinetic_mode(view: *mut ChamplainViewSys, mode: bool);
    fn champlain_view_set_zoom_on_double_click(view: *mut ChamplainViewSys, value: bool);
    fn champlain_view_add_layer(view: *mut ChamplainViewSys, layer: *mut ChamplainLayerSys);
    fn champlain_view_remove_layer(view: *mut ChamplainViewSys, layer: *mut ChamplainLayerSys);
}

pub struct ChamplainView {
    ptr: *mut ChamplainViewSys,
}

impl ChamplainView {
    pub(crate) fn new_with_ptr(ptr: *mut ChamplainViewSys) -> Self {
        Self { ptr }
    }

    pub(crate) fn get_ptr(&mut self) -> *mut ChamplainViewSys {
        self.ptr
    }

    pub fn center_on(&mut self, latitude: f64, longitude: f64) {
        unsafe { champlain_view_center_on(self.get_ptr(), latitude, longitude) }
    }

    pub fn set_zoom_level(&mut self, zoom_level: u32) {
        unsafe { champlain_view_set_zoom_level(self.get_ptr(), zoom_level) }
    }

    pub fn set_kinetic_mode(&mut self, mode: bool) {
        unsafe { champlain_view_set_kinetic_mode(self.get_ptr(), mode) }
    }

    pub fn set_zoom_on_double_click(&mut self, value: bool) {
        unsafe { champlain_view_set_zoom_on_double_click(self.get_ptr(), value) }
    }

    pub fn add_layer(&mut self, layer: &mut ChamplainLayer) {
        unsafe { champlain_view_add_layer(self.get_ptr(), layer.get_ptr()) }
    }

    pub fn remove_layer(&mut self, layer: &mut ChamplainLayer) {
        unsafe { champlain_view_remove_layer(self.get_ptr(), layer.get_ptr()) }
    }

    pub fn set_reactive(&mut self, reactive: bool) {
        let mut clutter_actor = ClutterActor::new(self.get_ptr() as *mut ClutterActorSys);
        clutter_actor.set_reactive(reactive)
    }
}
