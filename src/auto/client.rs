// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::{GlobalProxy,Object,PipewireObject,Proxy};
use glib::{translate::*};

glib::wrapper! {
    #[doc(alias = "WpClient")]
    pub struct Client(Object<ffi::WpClient, ffi::WpClientClass>) @extends GlobalProxy, Proxy, Object, @implements PipewireObject;

    match fn {
        type_ => || ffi::wp_client_get_type(),
    }
}

impl Client {
    #[doc(alias = "wp_client_send_error")]
    pub fn send_error(&self, id: u32, res: i32, message: &str) {
        unsafe {
            ffi::wp_client_send_error(self.to_glib_none().0, id, res, message.to_glib_none().0);
        }
    }
}
