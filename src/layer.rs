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
use crate::view::{ChamplainView, ChamplainViewSys};

#[repr(C)]
pub struct ChamplainLayer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ChamplainBoundingBox {
    _private: [u8; 0],
}

pub fn to_clutter_actor(input: *mut ChamplainLayer) -> ClutterActor {
    unsafe { ClutterActor::new(&mut *(input as *mut ClutterActorSys)) }
}

/// ChamplainLayer functions
#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_layer_set_view(layer: *mut ChamplainLayer, view: *mut ChamplainViewSys);
    fn champlain_layer_get_bounding_box(layer: *mut ChamplainLayer) -> *mut ChamplainBoundingBox;
}

pub fn set_view(layer: *mut ChamplainLayer, view: &ChamplainView) {
    unsafe { champlain_layer_set_view(layer, view.get_ptr()) }
}

pub fn get_bounding_box(layer: *mut ChamplainLayer) -> *mut ChamplainBoundingBox {
    unsafe { champlain_layer_get_bounding_box(layer) }
}
