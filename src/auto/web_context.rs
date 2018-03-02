// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use CacheModel;
use CookieManager;
use Download;
use FaviconDatabase;
#[cfg(any(feature = "v2_4", feature = "dox"))]
use ProcessModel;
use SecurityManager;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use SecurityOrigin;
use TLSErrorsPolicy;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use WebsiteDataManager;
use ffi;
#[cfg(any(feature = "v2_6", feature = "dox"))]
use gio;
use glib;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct WebContext(Object<ffi::WebKitWebContext, ffi::WebKitWebContextClass>);

    match fn {
        get_type => || ffi::webkit_web_context_get_type(),
    }
}

impl WebContext {
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    pub fn new() -> WebContext {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_web_context_new())
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    pub fn new_ephemeral() -> WebContext {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_web_context_new_ephemeral())
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    pub fn new_with_website_data_manager(manager: &WebsiteDataManager) -> WebContext {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::webkit_web_context_new_with_website_data_manager(manager.to_glib_none().0))
        }
    }

    pub fn get_default() -> Option<WebContext> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_default())
        }
    }
}

#[cfg(any(feature = "v2_8", feature = "dox"))]
impl Default for WebContext {
    fn default() -> Self {
        Self::new()
    }
}

pub trait WebContextExt {
    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn allow_tls_certificate_for_host(&self, certificate: &gio::TlsCertificate, host: &str);

    fn clear_cache(&self);

    fn download_uri(&self, uri: &str) -> Option<Download>;

    fn get_cache_model(&self) -> CacheModel;

    fn get_cookie_manager(&self) -> Option<CookieManager>;

    fn get_favicon_database(&self) -> Option<FaviconDatabase>;

    fn get_favicon_database_directory(&self) -> Option<String>;

    //fn get_plugins<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R);

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    fn get_process_model(&self) -> ProcessModel;

    fn get_security_manager(&self) -> Option<SecurityManager>;

    fn get_spell_checking_enabled(&self) -> bool;

    fn get_spell_checking_languages(&self) -> Vec<String>;

    fn get_tls_errors_policy(&self) -> TLSErrorsPolicy;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_web_process_count_limit(&self) -> u32;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_website_data_manager(&self) -> Option<WebsiteDataManager>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn initialize_notification_permissions(&self, allowed_origins: &[&SecurityOrigin], disallowed_origins: &[&SecurityOrigin]);

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    fn is_automation_allowed(&self) -> bool;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn is_ephemeral(&self) -> bool;

    fn prefetch_dns(&self, hostname: &str);

    //fn register_uri_scheme<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, scheme: &str, callback: /*Unknown conversion*//*Unimplemented*/URISchemeRequestCallback, user_data: P, user_data_destroy_func: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_additional_plugins_directory(&self, directory: &str);

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    fn set_automation_allowed(&self, allowed: bool);

    fn set_cache_model(&self, cache_model: CacheModel);

    #[cfg_attr(feature = "v2_10", deprecated)]
    fn set_disk_cache_directory(&self, directory: &str);

    fn set_favicon_database_directory<'a, P: Into<Option<&'a str>>>(&self, path: P);

    fn set_preferred_languages(&self, languages: &[&str]);

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    fn set_process_model(&self, process_model: ProcessModel);

    fn set_spell_checking_enabled(&self, enabled: bool);

    fn set_spell_checking_languages(&self, languages: &[&str]);

    fn set_tls_errors_policy(&self, policy: TLSErrorsPolicy);

    fn set_web_extensions_directory(&self, directory: &str);

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    fn set_web_extensions_initialization_user_data(&self, user_data: &glib::Variant);

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn set_web_process_count_limit(&self, limit: u32);

    #[cfg_attr(feature = "v2_10", deprecated)]
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_property_local_storage_directory(&self) -> Option<String>;

    //#[cfg(any(feature = "v2_18", feature = "dox"))]
    //fn connect_automation_started<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_download_started<F: Fn(&Self, &Download) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_initialize_notification_permissions<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    fn connect_initialize_web_extensions<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v2_10", deprecated)]
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_property_local_storage_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_website_data_manager_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebContext> + IsA<glib::object::Object>> WebContextExt for O {
    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn allow_tls_certificate_for_host(&self, certificate: &gio::TlsCertificate, host: &str) {
        unsafe {
            ffi::webkit_web_context_allow_tls_certificate_for_host(self.to_glib_none().0, certificate.to_glib_none().0, host.to_glib_none().0);
        }
    }

    fn clear_cache(&self) {
        unsafe {
            ffi::webkit_web_context_clear_cache(self.to_glib_none().0);
        }
    }

    fn download_uri(&self, uri: &str) -> Option<Download> {
        unsafe {
            from_glib_full(ffi::webkit_web_context_download_uri(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    fn get_cache_model(&self) -> CacheModel {
        unsafe {
            from_glib(ffi::webkit_web_context_get_cache_model(self.to_glib_none().0))
        }
    }

    fn get_cookie_manager(&self) -> Option<CookieManager> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_cookie_manager(self.to_glib_none().0))
        }
    }

    fn get_favicon_database(&self) -> Option<FaviconDatabase> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_favicon_database(self.to_glib_none().0))
        }
    }

    fn get_favicon_database_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_favicon_database_directory(self.to_glib_none().0))
        }
    }

    //fn get_plugins<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_web_context_get_plugins() }
    //}

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    fn get_process_model(&self) -> ProcessModel {
        unsafe {
            from_glib(ffi::webkit_web_context_get_process_model(self.to_glib_none().0))
        }
    }

    fn get_security_manager(&self) -> Option<SecurityManager> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_security_manager(self.to_glib_none().0))
        }
    }

    fn get_spell_checking_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_context_get_spell_checking_enabled(self.to_glib_none().0))
        }
    }

    fn get_spell_checking_languages(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_web_context_get_spell_checking_languages(self.to_glib_none().0))
        }
    }

    fn get_tls_errors_policy(&self) -> TLSErrorsPolicy {
        unsafe {
            from_glib(ffi::webkit_web_context_get_tls_errors_policy(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_web_process_count_limit(&self) -> u32 {
        unsafe {
            ffi::webkit_web_context_get_web_process_count_limit(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_website_data_manager(&self) -> Option<WebsiteDataManager> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_website_data_manager(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn initialize_notification_permissions(&self, allowed_origins: &[&SecurityOrigin], disallowed_origins: &[&SecurityOrigin]) {
        unsafe {
            ffi::webkit_web_context_initialize_notification_permissions(self.to_glib_none().0, allowed_origins.to_glib_none().0, disallowed_origins.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    fn is_automation_allowed(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_context_is_automation_allowed(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn is_ephemeral(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_context_is_ephemeral(self.to_glib_none().0))
        }
    }

    fn prefetch_dns(&self, hostname: &str) {
        unsafe {
            ffi::webkit_web_context_prefetch_dns(self.to_glib_none().0, hostname.to_glib_none().0);
        }
    }

    //fn register_uri_scheme<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, scheme: &str, callback: /*Unknown conversion*//*Unimplemented*/URISchemeRequestCallback, user_data: P, user_data_destroy_func: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::webkit_web_context_register_uri_scheme() }
    //}

    fn set_additional_plugins_directory(&self, directory: &str) {
        unsafe {
            ffi::webkit_web_context_set_additional_plugins_directory(self.to_glib_none().0, directory.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    fn set_automation_allowed(&self, allowed: bool) {
        unsafe {
            ffi::webkit_web_context_set_automation_allowed(self.to_glib_none().0, allowed.to_glib());
        }
    }

    fn set_cache_model(&self, cache_model: CacheModel) {
        unsafe {
            ffi::webkit_web_context_set_cache_model(self.to_glib_none().0, cache_model.to_glib());
        }
    }

    fn set_disk_cache_directory(&self, directory: &str) {
        unsafe {
            ffi::webkit_web_context_set_disk_cache_directory(self.to_glib_none().0, directory.to_glib_none().0);
        }
    }

    fn set_favicon_database_directory<'a, P: Into<Option<&'a str>>>(&self, path: P) {
        let path = path.into();
        let path = path.to_glib_none();
        unsafe {
            ffi::webkit_web_context_set_favicon_database_directory(self.to_glib_none().0, path.0);
        }
    }

    fn set_preferred_languages(&self, languages: &[&str]) {
        unsafe {
            ffi::webkit_web_context_set_preferred_languages(self.to_glib_none().0, languages.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    fn set_process_model(&self, process_model: ProcessModel) {
        unsafe {
            ffi::webkit_web_context_set_process_model(self.to_glib_none().0, process_model.to_glib());
        }
    }

    fn set_spell_checking_enabled(&self, enabled: bool) {
        unsafe {
            ffi::webkit_web_context_set_spell_checking_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    fn set_spell_checking_languages(&self, languages: &[&str]) {
        unsafe {
            ffi::webkit_web_context_set_spell_checking_languages(self.to_glib_none().0, languages.to_glib_none().0);
        }
    }

    fn set_tls_errors_policy(&self, policy: TLSErrorsPolicy) {
        unsafe {
            ffi::webkit_web_context_set_tls_errors_policy(self.to_glib_none().0, policy.to_glib());
        }
    }

    fn set_web_extensions_directory(&self, directory: &str) {
        unsafe {
            ffi::webkit_web_context_set_web_extensions_directory(self.to_glib_none().0, directory.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    fn set_web_extensions_initialization_user_data(&self, user_data: &glib::Variant) {
        unsafe {
            ffi::webkit_web_context_set_web_extensions_initialization_user_data(self.to_glib_none().0, user_data.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn set_web_process_count_limit(&self, limit: u32) {
        unsafe {
            ffi::webkit_web_context_set_web_process_count_limit(self.to_glib_none().0, limit);
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_property_local_storage_directory(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "local-storage-directory".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    //#[cfg(any(feature = "v2_18", feature = "dox"))]
    //fn connect_automation_started<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored session: WebKit2.AutomationSession
    //}

    fn connect_download_started<F: Fn(&Self, &Download) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Download) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "download-started",
                transmute(download_started_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_initialize_notification_permissions<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "initialize-notification-permissions",
                transmute(initialize_notification_permissions_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    fn connect_initialize_web_extensions<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "initialize-web-extensions",
                transmute(initialize_web_extensions_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_property_local_storage_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::local-storage-directory",
                transmute(notify_local_storage_directory_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_website_data_manager_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::website-data-manager",
                transmute(notify_website_data_manager_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn download_started_trampoline<P>(this: *mut ffi::WebKitWebContext, download: *mut ffi::WebKitDownload, f: glib_ffi::gpointer)
where P: IsA<WebContext> {
    callback_guard!();
    let f: &&(Fn(&P, &Download) + 'static) = transmute(f);
    f(&WebContext::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(download))
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
unsafe extern "C" fn initialize_notification_permissions_trampoline<P>(this: *mut ffi::WebKitWebContext, f: glib_ffi::gpointer)
where P: IsA<WebContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_4", feature = "dox"))]
unsafe extern "C" fn initialize_web_extensions_trampoline<P>(this: *mut ffi::WebKitWebContext, f: glib_ffi::gpointer)
where P: IsA<WebContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_8", feature = "dox"))]
unsafe extern "C" fn notify_local_storage_directory_trampoline<P>(this: *mut ffi::WebKitWebContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
unsafe extern "C" fn notify_website_data_manager_trampoline<P>(this: *mut ffi::WebKitWebContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebContext::from_glib_borrow(this).downcast_unchecked())
}
