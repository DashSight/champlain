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

use crate::view::{ChamplainView, ChamplainViewSys};
use glib::translate::FromGlibPtrNone;
use glib::translate::ToGlibPtr;

#[link(name = "champlain-gtk-0.12")]
extern "C" {
    fn gtk_champlain_embed_new() -> *mut gtk_sys::GtkWidget;
    fn gtk_champlain_embed_get_view(embed: *mut gtk_sys::GtkWidget) -> *mut ChamplainViewSys;
}

pub fn new() -> gtk::Widget {
    unsafe { gtk::Widget::from_glib_none(gtk_champlain_embed_new()) }
}

pub fn get_view(embed: gtk::Widget) -> ChamplainView {
    unsafe { ChamplainView::new_with_ptr(gtk_champlain_embed_get_view(embed.to_glib_none().0)) }
}
