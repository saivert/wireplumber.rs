// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::{Core,GlobalProxy,Object,PipewireObject,Properties,Proxy};
#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
use crate::{LinkState};
use glib::{translate::*};
#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId}};
use std::{mem};
#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
use std::{boxed::Box as Box_,mem::transmute,ptr};

glib::wrapper! {
    #[doc(alias = "WpLink")]
    pub struct Link(Object<ffi::WpLink, ffi::WpLinkClass>) @extends GlobalProxy, Proxy, Object, @implements PipewireObject;

    match fn {
        type_ => || ffi::wp_link_get_type(),
    }
}

impl Link {
    #[doc(alias = "wp_link_new_from_factory")]
    #[doc(alias = "new_from_factory")]
    pub fn from_factory(core: &Core, factory_name: &str, properties: Option<Properties>) -> Option<Link> {
        unsafe {
            from_glib_full(ffi::wp_link_new_from_factory(core.to_glib_none().0, factory_name.to_glib_none().0, properties.into_glib_ptr()))
        }
    }

    #[doc(alias = "wp_link_get_linked_object_ids")]
    #[doc(alias = "get_linked_object_ids")]
    pub fn linked_object_ids(&self) -> (u32, u32, u32, u32) {
        unsafe {
            let mut output_node = mem::MaybeUninit::uninit();
            let mut output_port = mem::MaybeUninit::uninit();
            let mut input_node = mem::MaybeUninit::uninit();
            let mut input_port = mem::MaybeUninit::uninit();
            ffi::wp_link_get_linked_object_ids(self.to_glib_none().0, output_node.as_mut_ptr(), output_port.as_mut_ptr(), input_node.as_mut_ptr(), input_port.as_mut_ptr());
            (output_node.assume_init(), output_port.assume_init(), input_node.assume_init(), input_port.assume_init())
        }
    }

    #[cfg(any(feature = "v0_4_11", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
    #[doc(alias = "wp_link_get_state")]
    #[doc(alias = "get_state")]
    pub fn state(&self) -> (LinkState, glib::GString) {
        unsafe {
            let mut error = ptr::null();
            let ret = from_glib(ffi::wp_link_get_state(self.to_glib_none().0, &mut error));
            (ret, from_glib_none(error))
        }
    }

    #[cfg(any(feature = "v0_4_11", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
    #[doc(alias = "state-changed")]
    pub fn connect_state_changed<F: Fn(&Self, LinkState, LinkState) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn state_changed_trampoline<F: Fn(&Link, LinkState, LinkState) + 'static>(this: *mut ffi::WpLink, object: ffi::WpLinkState, p0: ffi::WpLinkState, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(object), from_glib(p0))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"state-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(state_changed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_4_11", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
    #[doc(alias = "state")]
    pub fn connect_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<F: Fn(&Link) + 'static>(this: *mut ffi::WpLink, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_state_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}
