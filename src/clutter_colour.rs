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

use libc::c_uint;

#[repr(C)]
pub struct ClutterColor {
    _private: [u8; 0],
}

#[link(name = "champlain-0.12")]
extern "C" {
    fn clutter_color_new(
        red: c_uint,
        green: c_uint,
        blue: c_uint,
        alpha: c_uint,
    ) -> *const ClutterColor;
}

pub fn new(red: c_uint, green: c_uint, blue: c_uint, alpha: c_uint) -> *const ClutterColor {
    unsafe { clutter_color_new(red, green, blue, alpha) }
}
