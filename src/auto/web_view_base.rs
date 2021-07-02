// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitWebViewBase")]
    pub struct WebViewBase(Object<ffi::WebKitWebViewBase, ffi::WebKitWebViewBaseClass>) @extends gtk::Widget;

    match fn {
        type_ => || ffi::webkit_web_view_base_get_type(),
    }
}

impl WebViewBase {}

pub const NONE_WEB_VIEW_BASE: Option<&WebViewBase> = None;

impl fmt::Display for WebViewBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebViewBase")
    }
}
