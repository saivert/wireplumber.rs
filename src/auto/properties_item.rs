// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

#[cfg(any(feature = "v0_4_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_2")))]
use glib::{translate::*};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PropertiesItem(Shared<ffi::WpPropertiesItem>);

    match fn {
        ref => |ptr| ffi::wp_properties_item_ref(ptr),
        unref => |ptr| ffi::wp_properties_item_unref(ptr),
        type_ => || ffi::wp_properties_item_get_type(),
    }
}

impl PropertiesItem {
    #[cfg(any(feature = "v0_4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_2")))]
    #[doc(alias = "wp_properties_item_get_key")]
    #[doc(alias = "get_key")]
    pub fn key(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_properties_item_get_key(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_2")))]
    #[doc(alias = "wp_properties_item_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_properties_item_get_value(self.to_glib_none().0))
        }
    }
}
