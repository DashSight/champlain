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

use crate::clutter::ClutterActor;
use crate::layer::ChamplainLayer;
use crate::location::ChamplainLocation;
use crate::marker::ChamplainMarker;
use crate::point::ChamplainPoint;

pub fn to_champlain_layer(input: *mut ClutterActor) -> *mut ChamplainLayer {
    unsafe { &mut *(input as *mut ChamplainLayer) }
}

pub fn to_champlain_marker(input: *mut ClutterActor) -> *mut ChamplainMarker {
    unsafe { &mut *(input as *mut ChamplainMarker) }
}

pub fn to_location(actor: *mut ClutterActor) -> *mut ChamplainLocation {
    unsafe { std::mem::transmute::<*mut ClutterActor, *mut ChamplainLocation>(actor) }
}

pub fn to_point(input: *mut ClutterActor) -> *mut ChamplainPoint {
    unsafe { &mut *(input as *mut ChamplainPoint) }
}

#[link(name = "clutter-1.0")]
extern "C" {
    fn clutter_actor_add_child(me: *mut ClutterActor, child: *mut ClutterActor);
    fn clutter_actor_set_reactive(actor: *mut ClutterActor, reactive: bool);
    fn clutter_actor_show(me: *mut ClutterActor);
}

pub fn actor_add_child(me: *mut ClutterActor, child: *mut ClutterActor) {
    unsafe { clutter_actor_add_child(me, child) }
}

pub fn set_reactive(actor: *mut ClutterActor, reactive: bool) {
    unsafe { clutter_actor_set_reactive(actor, reactive) }
}

pub fn show(me: *mut ClutterActor) {
    unsafe { clutter_actor_show(me) }
}
