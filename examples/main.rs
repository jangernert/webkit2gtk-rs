/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

 extern crate gtk4;
 extern crate webkit2gtk;
 
 use gtk4::prelude::*;
 use webkit2gtk::{traits::{WebContextExt, WebViewExt}, WebContext, WebView};
 
 fn main() {
     let application =
         gtk4::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());
     application.connect_activate(build_ui);
     application.run();
 }
 
 fn build_ui(application: &gtk4::Application) {
     let window = gtk4::ApplicationWindow::new(application);
 
     window.set_title(Some("First GTK Program"));
     window.set_default_size(350, 70);
 
     let context = WebContext::default().unwrap();
     let webview = WebView::with_context(&context);
     webview.load_uri("https://crates.io/");
     window.set_child(Some(&webview));
 
     window.show();
 }