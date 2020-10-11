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

pub struct ChamplainLayer {
    ptr: *mut ChamplainLayerSys,
}

impl ChamplainLayer {
    pub(crate) fn new(ptr: *mut ChamplainLayerSys) -> Self {
        Self { ptr }
    }

    pub(crate) fn get_ptr(&self) -> *mut ChamplainLayerSys {
        self.ptr
    }

    // TODO: This is unsafe as now we have two copied of self.get_ptr()
    pub fn to_clutter_actor(&self) -> ClutterActor {
        unsafe { ClutterActor::new(&mut *(self.get_ptr() as *mut ClutterActorSys)) }
    }
}

#[repr(C)]
pub struct ChamplainLayerSys {
    _private: [u8; 0],
}

/// ChamplainLayer functions
#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_layer_set_view(layer: *mut ChamplainLayerSys, view: *mut ChamplainViewSys);
}

pub fn set_view(layer: &mut ChamplainLayer, view: &ChamplainView) {
    unsafe { champlain_layer_set_view(layer.get_ptr(), view.get_ptr()) }
}
