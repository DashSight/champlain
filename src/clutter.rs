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

use crate::view::*;

#[repr(C)]
#[derive(Clone)]
pub struct ClutterActor {
    _private: [u8; 0],
}

#[link(name = "clutter-1.0")]
extern "C" {
    fn clutter_stage_new() -> *mut ClutterActor;
    fn clutter_actor_add_child(me: *mut ClutterActor, child: *mut ClutterActor);
}

pub fn clutter_actor(input: *mut ChamplainView) -> *mut ClutterActor {
    unsafe { &mut *(input as *mut ClutterActor) }
}

pub fn stage_new() -> *mut ClutterActor {
    unsafe { clutter_stage_new() }
}

pub fn actor_add_child(me: *mut ClutterActor, child: *mut ClutterActor) {
    unsafe { clutter_actor_add_child(me, child) }
}
