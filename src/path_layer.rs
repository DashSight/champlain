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

#[repr(C)]
pub struct ChamplainPathLayer {
    _private: [u8; 0],
}

pub fn to_layer(input: *mut ChamplainPathLayer) -> *mut ChamplainLayer {
    unsafe { &mut *(input as *mut ChamplainLayer) }
}

#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_path_layer_new() -> *mut ChamplainPathLayer;
    fn champlain_path_layer_add_node(
        layer: *mut ChamplainPathLayer,
        location: *mut ChamplainLocation,
    );
    fn champlain_path_layer_remove_node(
        layer: *mut ChamplainPathLayer,
        location: *mut ChamplainLocation,
    );
    fn champlain_path_layer_set_visible(layer: *mut ChamplainPathLayer, value: bool);
}

pub fn new() -> *mut ChamplainPathLayer {
    unsafe { champlain_path_layer_new() }
}

pub fn add_node(layer: *mut ChamplainPathLayer, location: *mut ChamplainLocation) {
    unsafe { champlain_path_layer_add_node(layer, location) }
}

pub fn remove_node(layer: *mut ChamplainPathLayer, location: *mut ChamplainLocation) {
    unsafe { champlain_path_layer_remove_node(layer, location) }
}

pub fn set_visible(layer: *mut ChamplainPathLayer, value: bool) {
    unsafe { champlain_path_layer_set_visible(layer, value) }
}