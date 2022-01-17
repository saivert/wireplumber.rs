// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::GlobalProxy;
use crate::Iterator;
use crate::Object;
use crate::Proxy;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WpMetadata")]
    pub struct Metadata(Object<ffi::WpMetadata, ffi::WpMetadataClass>) @extends GlobalProxy, Proxy, Object;

    match fn {
        type_ => || ffi::wp_metadata_get_type(),
    }
}

impl Metadata {
        pub const NONE: Option<&'static Metadata> = None;
    

    #[doc(alias = "wp_metadata_iterator_item_extract")]
    pub fn iterator_item_extract(item: &glib::Value) -> (u32, glib::GString, glib::GString, glib::GString) {
        unsafe {
            let mut subject = mem::MaybeUninit::uninit();
            let mut key = ptr::null();
            let mut type_ = ptr::null();
            let mut value = ptr::null();
            ffi::wp_metadata_iterator_item_extract(item.to_glib_none().0, subject.as_mut_ptr(), &mut key, &mut type_, &mut value);
            let subject = subject.assume_init();
            (subject, from_glib_none(key), from_glib_none(type_), from_glib_none(value))
        }
    }
}

pub trait MetadataExt: 'static {
    #[doc(alias = "wp_metadata_clear")]
    fn clear(&self);

    #[doc(alias = "wp_metadata_find")]
    fn find(&self, subject: u32, key: &str) -> (glib::GString, glib::GString);

    #[doc(alias = "wp_metadata_new_iterator")]
    fn new_iterator(&self, subject: u32) -> Option<Iterator>;

    #[doc(alias = "wp_metadata_set")]
    fn set(&self, subject: u32, key: Option<&str>, type_: Option<&str>, value: Option<&str>);

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self, u32, &str, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Metadata>> MetadataExt for O {
    fn clear(&self) {
        unsafe {
            ffi::wp_metadata_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn find(&self, subject: u32, key: &str) -> (glib::GString, glib::GString) {
        unsafe {
            let mut type_ = ptr::null();
            let ret = from_glib_none(ffi::wp_metadata_find(self.as_ref().to_glib_none().0, subject, key.to_glib_none().0, &mut type_));
            (ret, from_glib_full(type_))
        }
    }

    fn new_iterator(&self, subject: u32) -> Option<Iterator> {
        unsafe {
            from_glib_full(ffi::wp_metadata_new_iterator(self.as_ref().to_glib_none().0, subject))
        }
    }

    fn set(&self, subject: u32, key: Option<&str>, type_: Option<&str>, value: Option<&str>) {
        unsafe {
            ffi::wp_metadata_set(self.as_ref().to_glib_none().0, subject, key.to_glib_none().0, type_.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self, u32, &str, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P: IsA<Metadata>, F: Fn(&P, u32, &str, &str, &str) + 'static>(this: *mut ffi::WpMetadata, object: libc::c_uint, p0: *mut libc::c_char, p1: *mut libc::c_char, p2: *mut libc::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Metadata::from_glib_borrow(this).unsafe_cast_ref(), object, &glib::GString::from_glib_borrow(p0), &glib::GString::from_glib_borrow(p1), &glib::GString::from_glib_borrow(p2))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}
