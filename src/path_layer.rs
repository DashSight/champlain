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

use crate::clutter_colour::ClutterColor;
use crate::layer::{ChamplainLayer, ChamplainLayerSys};
use crate::location::{ChamplainLocation, ChamplainLocationSys};

#[repr(C)]
pub struct ChamplainPathLayerSys {
    _private: [u8; 0],
}

#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_path_layer_new() -> *mut ChamplainPathLayerSys;
    fn champlain_path_layer_add_node(
        layer: *mut ChamplainPathLayerSys,
        location: *mut ChamplainLocationSys,
    );
    fn champlain_path_layer_remove_node(
        layer: *mut ChamplainPathLayerSys,
        location: *mut ChamplainLocationSys,
    );
    fn champlain_path_layer_remove_all(layer: *mut ChamplainPathLayerSys);
    fn champlain_path_layer_set_visible(layer: *mut ChamplainPathLayerSys, value: bool);
    fn champlain_path_layer_set_stroke_color(
        layer: *mut ChamplainPathLayerSys,
        colour: *const ClutterColor,
    );
}

pub struct ChamplainPathLayer {
    ptr: *mut ChamplainPathLayerSys,
}

impl ChamplainPathLayer {
    pub(crate) fn new_with_ptr(ptr: *mut ChamplainPathLayerSys) -> Self {
        Self { ptr }
    }

    pub(crate) fn get_ptr(&self) -> *mut ChamplainPathLayerSys {
        self.ptr
    }

    // TODO: This is unsafe as now we have two copied of self.get_ptr()
    pub fn to_layer(&self) -> ChamplainLayer {
        unsafe { ChamplainLayer::new(&mut *(self.get_ptr() as *mut ChamplainLayerSys)) }
    }

    pub fn new() -> Self {
        unsafe { Self::new_with_ptr(champlain_path_layer_new()) }
    }

    pub fn add_node(&mut self, location: &mut ChamplainLocation) {
        unsafe { champlain_path_layer_add_node(self.get_ptr(), location.get_ptr()) }
    }

    pub fn remove_node(&mut self, location: &mut ChamplainLocation) {
        unsafe { champlain_path_layer_remove_node(self.get_ptr(), location.get_ptr()) }
    }

    pub fn remove_all(&mut self) {
        unsafe { champlain_path_layer_remove_all(self.get_ptr()) }
    }

    pub fn set_visible(&mut self, value: bool) {
        unsafe { champlain_path_layer_set_visible(self.get_ptr(), value) }
    }

    pub fn set_stroke_colour(&mut self, colour: *const ClutterColor) {
        unsafe { champlain_path_layer_set_stroke_color(self.get_ptr(), colour) }
    }
}
