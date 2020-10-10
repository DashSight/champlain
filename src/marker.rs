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
pub struct ChamplainMarker {
    _private: [u8; 0],
}

/// ChamplainMarker functions
#[link(name = "champlain-0.12")]
extern "C" {
    fn champlain_marker_new() -> *mut ClutterActorSys;
    fn champlain_marker_animate_in(marker: *mut ChamplainMarker);
}

pub fn new() -> ClutterActor {
    unsafe { ClutterActor::new(champlain_marker_new()) }
}

pub fn animate_in(marker: *mut ChamplainMarker) {
    unsafe { champlain_marker_animate_in(marker) }
}
