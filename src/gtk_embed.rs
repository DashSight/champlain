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

#[link(name = "champlain-gtk-0.12")]
extern "C" {
    fn gtk_champlain_embed_new() -> gtk::Widget;
    fn gtk_champlain_embed_get_view(embed: gtk::Widget) -> *mut ChamplainView;
}

pub fn new() -> gtk::Widget {
    unsafe { gtk_champlain_embed_new() }
}

pub fn get_view(embed: gtk::Widget) -> Option<*mut ChamplainView> {
    unsafe { Some(gtk_champlain_embed_get_view(embed)) }
}
