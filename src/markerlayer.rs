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

use crate::marker::ChamplainMarker;

#[repr(C)]
pub struct ChamplainMarkerLayer {
    _private: [u8; 0],
}

#[repr(C)]
pub enum ChamplainSelectionMode {
    /// No marker can be selected.
    ChamplainSelectionNone,
    /// Only one marker can be selected.
    ChamplainSelectionSingle,
    /// Multiple marker can be selected.
    ChamplainSelectionMultiple,
}

/// ChamplainMarkerLayer functions
#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_marker_layer_new() -> *mut ChamplainMarkerLayer;
    fn champlain_marker_layer_new_full(mode: ChamplainSelectionMode) -> *mut ChamplainMarkerLayer;
    fn champlain_marker_layer_add_marker(
        layer: *mut ChamplainMarkerLayer,
        marker: *mut ChamplainMarker,
    );
    fn champlain_marker_layer_animate_in_all_markers(layer: *mut ChamplainMarkerLayer);
}

pub fn new() -> *mut ChamplainMarkerLayer {
    unsafe { champlain_marker_layer_new() }
}

pub fn new_full(mode: ChamplainSelectionMode) -> *mut ChamplainMarkerLayer {
    unsafe { champlain_marker_layer_new_full(mode) }
}

pub fn add_marker(layer: *mut ChamplainMarkerLayer, marker: *mut ChamplainMarker) {
    unsafe { champlain_marker_layer_add_marker(layer, marker) }
}

pub fn animate_in_all_markers(layer: *mut ChamplainMarkerLayer) {
    unsafe { champlain_marker_layer_animate_in_all_markers(layer) }
}
