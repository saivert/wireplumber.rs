// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::ConstraintType;
use crate::ConstraintVerb;
use crate::InterestMatch;
use crate::InterestMatchFlags;
use crate::Properties;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectInterest(Shared<ffi::WpObjectInterest>);

    match fn {
        ref => |ptr| ffi::wp_object_interest_ref(ptr),
        unref => |ptr| ffi::wp_object_interest_unref(ptr),
        type_ => || ffi::wp_object_interest_get_type(),
    }
}

impl ObjectInterest {
    //#[doc(alias = "wp_object_interest_new")]
    //pub fn new(gtype: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> ObjectInterest {
    //    unsafe { TODO: call ffi:wp_object_interest_new() }
    //}

    #[doc(alias = "wp_object_interest_new_type")]
    pub fn new_type(gtype: glib::types::Type) -> ObjectInterest {
        unsafe {
            from_glib_full(ffi::wp_object_interest_new_type(gtype.into_glib()))
        }
    }

    //#[doc(alias = "wp_object_interest_new_valist")]
    //pub fn new_valist(gtype: glib::types::Type, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> ObjectInterest {
    //    unsafe { TODO: call ffi:wp_object_interest_new_valist() }
    //}

    #[doc(alias = "wp_object_interest_add_constraint")]
    pub fn add_constraint(&self, type_: ConstraintType, subject: &str, verb: ConstraintVerb, value: Option<&glib::Variant>) {
        unsafe {
            ffi::wp_object_interest_add_constraint(self.to_glib_none().0, type_.into_glib(), subject.to_glib_none().0, verb.into_glib(), value.to_glib_none().0);
        }
    }

    //#[doc(alias = "wp_object_interest_matches")]
    //pub fn matches(&self, object: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:wp_object_interest_matches() }
    //}

    #[doc(alias = "wp_object_interest_matches_full")]
    pub fn matches_full(&self, flags: InterestMatchFlags, object_type: glib::types::Type, object: Option<&impl IsA<glib::Object>>, pw_props: Option<&Properties>, pw_global_props: Option<&Properties>) -> InterestMatch {
        unsafe {
            from_glib(ffi::wp_object_interest_matches_full(self.to_glib_none().0, flags.into_glib(), object_type.into_glib(), object.map(|p| p.as_ref()).to_glib_none().0, pw_props.to_glib_none().0, pw_global_props.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_object_interest_validate")]
    pub fn validate(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::wp_object_interest_validate(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}
