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

#[repr(C)]
pub struct ChamplainMarkerSys {
    _private: [u8; 0],
}

/// ChamplainMarker functions
#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_marker_new() -> *mut ClutterActorSys;
    fn champlain_marker_animate_in(marker: *mut ChamplainMarkerSys);
}

pub struct ChamplainMarker {
    ptr: *mut ChamplainMarkerSys,
    actor: ClutterActor,
}

impl ChamplainMarker {
    pub(crate) fn new_with_ptr(ptr: *mut ChamplainMarkerSys) -> Self {
        Self {
            ptr,
            actor: ClutterActor::new(ptr as *mut ClutterActorSys),
        }
    }

    pub(crate) fn get_ptr(&mut self) -> *mut ChamplainMarkerSys {
        self.ptr
    }

    pub fn borrow_mut_actor(&mut self) -> &mut ClutterActor {
        &mut self.actor
    }

    pub fn new() -> Self {
        let marker = unsafe { champlain_marker_new() };
        let actor = marker as *mut ChamplainMarkerSys;
        Self::new_with_ptr(actor)
    }

    pub fn animate_in(&mut self) {
        unsafe { champlain_marker_animate_in(self.get_ptr()) }
    }
}
