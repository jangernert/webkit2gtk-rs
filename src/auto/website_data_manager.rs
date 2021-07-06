// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use crate::CookieManager;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
use crate::ITPThirdParty;
#[cfg(any(feature = "v2_32", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
use crate::NetworkProxyMode;
#[cfg(any(feature = "v2_32", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
use crate::NetworkProxySettings;
#[cfg(any(feature = "v2_32", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
use crate::TLSErrorsPolicy;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use crate::WebsiteData;
//#[cfg(any(feature = "v2_16", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use crate::WebsiteDataTypes;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
//#[cfg(any(feature = "v2_16", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use std::pin::Pin;
//#[cfg(any(feature = "v2_16", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WebKitWebsiteDataManager")]
    pub struct WebsiteDataManager(Object<ffi::WebKitWebsiteDataManager, ffi::WebKitWebsiteDataManagerClass>);

    match fn {
        type_ => || ffi::webkit_website_data_manager_get_type(),
    }
}

impl WebsiteDataManager {
    //#[doc(alias = "webkit_website_data_manager_new")]
    //pub fn new(first_option_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> WebsiteDataManager {
    //    unsafe { TODO: call ffi:webkit_website_data_manager_new() }
    //}

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_website_data_manager_new_ephemeral")]
    pub fn new_ephemeral() -> WebsiteDataManager {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_website_data_manager_new_ephemeral())
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`WebsiteDataManager`] objects.
            ///
            /// This method returns an instance of [`WebsiteDataManagerBuilder`] which can be used to create [`WebsiteDataManager`] objects.
            pub fn builder() -> WebsiteDataManagerBuilder {
                WebsiteDataManagerBuilder::default()
            }
        
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`WebsiteDataManager`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct WebsiteDataManagerBuilder {
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    base_cache_directory: Option<String>,
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    base_data_directory: Option<String>,
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    disk_cache_directory: Option<String>,
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    dom_cache_directory: Option<String>,
    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    hsts_cache_directory: Option<String>,
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    indexeddb_directory: Option<String>,
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    is_ephemeral: Option<bool>,
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    itp_directory: Option<String>,
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    local_storage_directory: Option<String>,
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    offline_application_cache_directory: Option<String>,
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    service_worker_registrations_directory: Option<String>,
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    #[cfg_attr(feature = "v2_24", deprecated = "Since 2.24")]
    websql_directory: Option<String>,
}

impl WebsiteDataManagerBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`WebsiteDataManagerBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`WebsiteDataManager`].
    pub fn build(self) -> WebsiteDataManager {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v2_10", feature = "dox"))]
if let Some(ref base_cache_directory) = self.base_cache_directory {
                properties.push(("base-cache-directory", base_cache_directory));
            }
        #[cfg(any(feature = "v2_10", feature = "dox"))]
if let Some(ref base_data_directory) = self.base_data_directory {
                properties.push(("base-data-directory", base_data_directory));
            }
        #[cfg(any(feature = "v2_10", feature = "dox"))]
if let Some(ref disk_cache_directory) = self.disk_cache_directory {
                properties.push(("disk-cache-directory", disk_cache_directory));
            }
        #[cfg(any(feature = "v2_30", feature = "dox"))]
if let Some(ref dom_cache_directory) = self.dom_cache_directory {
                properties.push(("dom-cache-directory", dom_cache_directory));
            }
        #[cfg(any(feature = "v2_26", feature = "dox"))]
if let Some(ref hsts_cache_directory) = self.hsts_cache_directory {
                properties.push(("hsts-cache-directory", hsts_cache_directory));
            }
        #[cfg(any(feature = "v2_10", feature = "dox"))]
if let Some(ref indexeddb_directory) = self.indexeddb_directory {
                properties.push(("indexeddb-directory", indexeddb_directory));
            }
        #[cfg(any(feature = "v2_16", feature = "dox"))]
if let Some(ref is_ephemeral) = self.is_ephemeral {
                properties.push(("is-ephemeral", is_ephemeral));
            }
        #[cfg(any(feature = "v2_30", feature = "dox"))]
if let Some(ref itp_directory) = self.itp_directory {
                properties.push(("itp-directory", itp_directory));
            }
        #[cfg(any(feature = "v2_10", feature = "dox"))]
if let Some(ref local_storage_directory) = self.local_storage_directory {
                properties.push(("local-storage-directory", local_storage_directory));
            }
        #[cfg(any(feature = "v2_10", feature = "dox"))]
if let Some(ref offline_application_cache_directory) = self.offline_application_cache_directory {
                properties.push(("offline-application-cache-directory", offline_application_cache_directory));
            }
        #[cfg(any(feature = "v2_30", feature = "dox"))]
if let Some(ref service_worker_registrations_directory) = self.service_worker_registrations_directory {
                properties.push(("service-worker-registrations-directory", service_worker_registrations_directory));
            }
        #[cfg(any(feature = "v2_10", feature = "dox"))]
if let Some(ref websql_directory) = self.websql_directory {
                properties.push(("websql-directory", websql_directory));
            }
        glib::Object::new::<WebsiteDataManager>(&properties)
                .expect("Failed to create an instance of WebsiteDataManager")

    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    pub fn base_cache_directory(mut self, base_cache_directory: &str) -> Self {
        self.base_cache_directory = Some(base_cache_directory.to_string());
        self
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    pub fn base_data_directory(mut self, base_data_directory: &str) -> Self {
        self.base_data_directory = Some(base_data_directory.to_string());
        self
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    pub fn disk_cache_directory(mut self, disk_cache_directory: &str) -> Self {
        self.disk_cache_directory = Some(disk_cache_directory.to_string());
        self
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    pub fn dom_cache_directory(mut self, dom_cache_directory: &str) -> Self {
        self.dom_cache_directory = Some(dom_cache_directory.to_string());
        self
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    pub fn hsts_cache_directory(mut self, hsts_cache_directory: &str) -> Self {
        self.hsts_cache_directory = Some(hsts_cache_directory.to_string());
        self
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    pub fn indexeddb_directory(mut self, indexeddb_directory: &str) -> Self {
        self.indexeddb_directory = Some(indexeddb_directory.to_string());
        self
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    pub fn is_ephemeral(mut self, is_ephemeral: bool) -> Self {
        self.is_ephemeral = Some(is_ephemeral);
        self
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    pub fn itp_directory(mut self, itp_directory: &str) -> Self {
        self.itp_directory = Some(itp_directory.to_string());
        self
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    pub fn local_storage_directory(mut self, local_storage_directory: &str) -> Self {
        self.local_storage_directory = Some(local_storage_directory.to_string());
        self
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    pub fn offline_application_cache_directory(mut self, offline_application_cache_directory: &str) -> Self {
        self.offline_application_cache_directory = Some(offline_application_cache_directory.to_string());
        self
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    pub fn service_worker_registrations_directory(mut self, service_worker_registrations_directory: &str) -> Self {
        self.service_worker_registrations_directory = Some(service_worker_registrations_directory.to_string());
        self
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    #[cfg_attr(feature = "v2_24", deprecated = "Since 2.24")]
    pub fn websql_directory(mut self, websql_directory: &str) -> Self {
        self.websql_directory = Some(websql_directory.to_string());
        self
    }
}

pub const NONE_WEBSITE_DATA_MANAGER: Option<&WebsiteDataManager> = None;

pub trait WebsiteDataManagerExt: 'static {
    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_website_data_manager_clear")]
    fn clear<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, types: WebsiteDataTypes, timespan: glib::TimeSpan, cancellable: Option<&P>, callback: Q);

    
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn clear_future(&self, types: WebsiteDataTypes, timespan: glib::TimeSpan) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_website_data_manager_fetch")]
    fn fetch<P: IsA<gio::Cancellable>, Q: FnOnce(Result<Vec<WebsiteData>, glib::Error>) + Send + 'static>(&self, types: WebsiteDataTypes, cancellable: Option<&P>, callback: Q);

    
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn fetch_future(&self, types: WebsiteDataTypes) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<WebsiteData>, glib::Error>> + 'static>>;

    #[doc(alias = "webkit_website_data_manager_get_base_cache_directory")]
    #[doc(alias = "get_base_cache_directory")]
    fn base_cache_directory(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_website_data_manager_get_base_data_directory")]
    #[doc(alias = "get_base_data_directory")]
    fn base_data_directory(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_website_data_manager_get_cookie_manager")]
    #[doc(alias = "get_cookie_manager")]
    fn cookie_manager(&self) -> Option<CookieManager>;

    #[doc(alias = "webkit_website_data_manager_get_disk_cache_directory")]
    #[doc(alias = "get_disk_cache_directory")]
    fn disk_cache_directory(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_data_manager_get_dom_cache_directory")]
    #[doc(alias = "get_dom_cache_directory")]
    fn dom_cache_directory(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    #[doc(alias = "webkit_website_data_manager_get_hsts_cache_directory")]
    #[doc(alias = "get_hsts_cache_directory")]
    fn hsts_cache_directory(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_website_data_manager_get_indexeddb_directory")]
    #[doc(alias = "get_indexeddb_directory")]
    fn indexeddb_directory(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_data_manager_get_itp_directory")]
    #[doc(alias = "get_itp_directory")]
    fn itp_directory(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_data_manager_get_itp_enabled")]
    #[doc(alias = "get_itp_enabled")]
    fn is_itp_enabled(&self) -> bool;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_data_manager_get_itp_summary")]
    #[doc(alias = "get_itp_summary")]
    fn itp_summary<P: IsA<gio::Cancellable>, Q: FnOnce(Result<Vec<ITPThirdParty>, glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q);

    
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn itp_summary_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<ITPThirdParty>, glib::Error>> + 'static>>;

    #[doc(alias = "webkit_website_data_manager_get_local_storage_directory")]
    #[doc(alias = "get_local_storage_directory")]
    fn local_storage_directory(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_website_data_manager_get_offline_application_cache_directory")]
    #[doc(alias = "get_offline_application_cache_directory")]
    fn offline_application_cache_directory(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_data_manager_get_persistent_credential_storage_enabled")]
    #[doc(alias = "get_persistent_credential_storage_enabled")]
    fn is_persistent_credential_storage_enabled(&self) -> bool;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_data_manager_get_service_worker_registrations_directory")]
    #[doc(alias = "get_service_worker_registrations_directory")]
    fn service_worker_registrations_directory(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "webkit_website_data_manager_get_tls_errors_policy")]
    #[doc(alias = "get_tls_errors_policy")]
    fn tls_errors_policy(&self) -> TLSErrorsPolicy;

    #[cfg_attr(feature = "v2_24", deprecated = "Since 2.24")]
    #[doc(alias = "webkit_website_data_manager_get_websql_directory")]
    #[doc(alias = "get_websql_directory")]
    fn websql_directory(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_website_data_manager_is_ephemeral")]
    fn is_ephemeral(&self) -> bool;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_data_manager_set_itp_enabled")]
    fn set_itp_enabled(&self, enabled: bool);

    // #[cfg(any(feature = "v2_32", feature = "dox"))]
    // #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    // #[doc(alias = "webkit_website_data_manager_set_network_proxy_settings")]
    // fn set_network_proxy_settings(&self, proxy_mode: NetworkProxyMode, proxy_settings: Option<&mut NetworkProxySettings>);

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_data_manager_set_persistent_credential_storage_enabled")]
    fn set_persistent_credential_storage_enabled(&self, enabled: bool);

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "webkit_website_data_manager_set_tls_errors_policy")]
    fn set_tls_errors_policy(&self, policy: TLSErrorsPolicy);
}

impl<O: IsA<WebsiteDataManager>> WebsiteDataManagerExt for O {
    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn clear<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, types: WebsiteDataTypes, timespan: glib::TimeSpan, cancellable: Option<&P>, callback: Q) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn clear_trampoline<Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_website_data_manager_clear_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = clear_trampoline::<Q>;
        unsafe {
            ffi::webkit_website_data_manager_clear(self.as_ref().to_glib_none().0, types.into_glib(), timespan, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn clear_future(&self, types: WebsiteDataTypes, timespan: glib::TimeSpan) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.clear(
                types,
                timespan,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn fetch<P: IsA<gio::Cancellable>, Q: FnOnce(Result<Vec<WebsiteData>, glib::Error>) + Send + 'static>(&self, types: WebsiteDataTypes, cancellable: Option<&P>, callback: Q) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn fetch_trampoline<Q: FnOnce(Result<Vec<WebsiteData>, glib::Error>) + Send + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_website_data_manager_fetch_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = fetch_trampoline::<Q>;
        unsafe {
            ffi::webkit_website_data_manager_fetch(self.as_ref().to_glib_none().0, types.into_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn fetch_future(&self, types: WebsiteDataTypes) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<WebsiteData>, glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.fetch(
                types,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    fn base_cache_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_base_cache_directory(self.as_ref().to_glib_none().0))
        }
    }

    fn base_data_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_base_data_directory(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn cookie_manager(&self) -> Option<CookieManager> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_cookie_manager(self.as_ref().to_glib_none().0))
        }
    }

    fn disk_cache_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_disk_cache_directory(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn dom_cache_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_dom_cache_directory(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    fn hsts_cache_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_hsts_cache_directory(self.as_ref().to_glib_none().0))
        }
    }

    fn indexeddb_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_indexeddb_directory(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn itp_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_itp_directory(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn is_itp_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_website_data_manager_get_itp_enabled(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn itp_summary<P: IsA<gio::Cancellable>, Q: FnOnce(Result<Vec<ITPThirdParty>, glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn itp_summary_trampoline<Q: FnOnce(Result<Vec<ITPThirdParty>, glib::Error>) + Send + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_website_data_manager_get_itp_summary_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = itp_summary_trampoline::<Q>;
        unsafe {
            ffi::webkit_website_data_manager_get_itp_summary(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn itp_summary_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<ITPThirdParty>, glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.itp_summary(
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    fn local_storage_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_local_storage_directory(self.as_ref().to_glib_none().0))
        }
    }

    fn offline_application_cache_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_offline_application_cache_directory(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn is_persistent_credential_storage_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_website_data_manager_get_persistent_credential_storage_enabled(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn service_worker_registrations_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_service_worker_registrations_directory(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    fn tls_errors_policy(&self) -> TLSErrorsPolicy {
        unsafe {
            from_glib(ffi::webkit_website_data_manager_get_tls_errors_policy(self.as_ref().to_glib_none().0))
        }
    }

    fn websql_directory(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_websql_directory(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn is_ephemeral(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_website_data_manager_is_ephemeral(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn set_itp_enabled(&self, enabled: bool) {
        unsafe {
            ffi::webkit_website_data_manager_set_itp_enabled(self.as_ref().to_glib_none().0, enabled.into_glib());
        }
    }

    // #[cfg(any(feature = "v2_32", feature = "dox"))]
    // #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    // fn set_network_proxy_settings(&self, proxy_mode: NetworkProxyMode, proxy_settings: Option<&mut NetworkProxySettings>) {
    //     unsafe {
    //         ffi::webkit_website_data_manager_set_network_proxy_settings(self.as_ref().to_glib_none().0, proxy_mode.into_glib(), proxy_settings.to_glib_none_mut().0);
    //     }
    // }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn set_persistent_credential_storage_enabled(&self, enabled: bool) {
        unsafe {
            ffi::webkit_website_data_manager_set_persistent_credential_storage_enabled(self.as_ref().to_glib_none().0, enabled.into_glib());
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    fn set_tls_errors_policy(&self, policy: TLSErrorsPolicy) {
        unsafe {
            ffi::webkit_website_data_manager_set_tls_errors_policy(self.as_ref().to_glib_none().0, policy.into_glib());
        }
    }
}

impl fmt::Display for WebsiteDataManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebsiteDataManager")
    }
}
