// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::{Direction,GlobalProxy,Object,PipewireObject,Proxy};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_,mem::transmute};

glib::wrapper! {
    #[doc(alias = "WpEndpoint")]
    pub struct Endpoint(Object<ffi::WpEndpoint, ffi::WpEndpointClass>) @extends GlobalProxy, Proxy, Object, @implements PipewireObject;

    match fn {
        type_ => || ffi::wp_endpoint_get_type(),
    }
}

impl Endpoint {
        pub const NONE: Option<&'static Endpoint> = None;
    
}

pub trait EndpointExt: 'static {
    #[doc(alias = "wp_endpoint_get_direction")]
    #[doc(alias = "get_direction")]
    fn direction(&self) -> Direction;

    #[doc(alias = "wp_endpoint_get_media_class")]
    #[doc(alias = "get_media_class")]
    fn media_class(&self) -> Option<glib::GString>;

    #[doc(alias = "wp_endpoint_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[doc(alias = "direction")]
    fn connect_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "media-class")]
    fn connect_media_class_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Endpoint>> EndpointExt for O {
    fn direction(&self) -> Direction {
        unsafe {
            from_glib(ffi::wp_endpoint_get_direction(self.as_ref().to_glib_none().0))
        }
    }

    fn media_class(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_endpoint_get_media_class(self.as_ref().to_glib_none().0))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_endpoint_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<P: IsA<Endpoint>, F: Fn(&P) + 'static>(this: *mut ffi::WpEndpoint, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Endpoint::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::direction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_direction_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_media_class_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_media_class_trampoline<P: IsA<Endpoint>, F: Fn(&P) + 'static>(this: *mut ffi::WpEndpoint, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Endpoint::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::media-class\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_media_class_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<Endpoint>, F: Fn(&P) + 'static>(this: *mut ffi::WpEndpoint, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Endpoint::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}
