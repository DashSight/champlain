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
use crate::marker::{ChamplainMarker, ChamplainMarkerSys};

#[repr(C)]
pub struct ChamplainMarkerLayerSys {
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
    fn champlain_marker_layer_new() -> *mut ChamplainMarkerLayerSys;
    fn champlain_marker_layer_new_full(
        mode: ChamplainSelectionMode,
    ) -> *mut ChamplainMarkerLayerSys;
    fn champlain_marker_layer_add_marker(
        layer: *mut ChamplainMarkerLayerSys,
        marker: *mut ChamplainMarkerSys,
    );
    fn champlain_marker_layer_remove_marker(
        layer: *mut ChamplainMarkerLayerSys,
        marker: *mut ChamplainMarkerSys,
    );
    fn champlain_marker_layer_remove_all(layer: *mut ChamplainMarkerLayerSys);
    fn champlain_marker_layer_animate_in_all_markers(layer: *mut ChamplainMarkerLayerSys);
    fn champlain_marker_layer_show_all_markers(layer: *mut ChamplainMarkerLayerSys);
}

pub struct ChamplainMarkerLayer {
    ptr: *mut ChamplainMarkerLayerSys,
    actor: ClutterActor,
    layer: ChamplainLayer,
}

impl ChamplainMarkerLayer {
    fn new_with_ptr(ptr: *mut ChamplainMarkerLayerSys) -> Self {
        Self {
            ptr,
            actor: ClutterActor::new(ptr as *mut ClutterActorSys),
            layer: ChamplainLayer::new_with_ptr(ptr as *mut ChamplainLayerSys),
        }
    }

    fn get_ptr(&mut self) -> *mut ChamplainMarkerLayerSys {
        self.ptr
    }

    pub fn borrow_mut_layer(&mut self) -> &mut ChamplainLayer {
        &mut self.layer
    }

    pub fn borrow_mut_actor(&mut self) -> &mut ClutterActor {
        &mut self.actor
    }

    pub fn new() -> Self {
        unsafe { Self::new_with_ptr(champlain_marker_layer_new()) }
    }

    pub fn new_full(mode: ChamplainSelectionMode) -> ChamplainMarkerLayer {
        unsafe { Self::new_with_ptr(champlain_marker_layer_new_full(mode)) }
    }

    pub fn add_marker(&mut self, marker: &mut ChamplainMarker) {
        unsafe { champlain_marker_layer_add_marker(self.get_ptr(), marker.get_ptr()) }
    }

    pub fn remove_marker(&mut self, marker: &mut ChamplainMarker) {
        unsafe { champlain_marker_layer_remove_marker(self.get_ptr(), marker.get_ptr()) }
    }

    pub fn remove_all(&mut self) {
        unsafe { champlain_marker_layer_remove_all(self.get_ptr()) }
    }

    pub fn animate_in_all_markers(&mut self) {
        unsafe { champlain_marker_layer_animate_in_all_markers(self.get_ptr()) }
    }

    pub fn show_all_markers(&mut self) {
        unsafe { champlain_marker_layer_show_all_markers(self.get_ptr()) }
    }
}
