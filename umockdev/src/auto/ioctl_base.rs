// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,IoctlClient};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "UMockdevIoctlBase")]
    pub struct IoctlBase(Object<ffi::UMockdevIoctlBase, ffi::UMockdevIoctlBaseClass>);

    match fn {
        type_ => || ffi::umockdev_ioctl_base_get_type(),
    }
}

impl IoctlBase {
        pub const NONE: Option<&'static IoctlBase> = None;
    

    #[doc(alias = "umockdev_ioctl_base_new")]
    pub fn new() -> IoctlBase {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::umockdev_ioctl_base_new())
        }
    }
}

impl Default for IoctlBase {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::IoctlBase>> Sealed for T {}
}

pub trait IoctlBaseExt: IsA<IoctlBase> + sealed::Sealed + 'static {
    #[doc(alias = "umockdev_ioctl_base_handle_ioctl")]
    fn handle_ioctl(&self, client: &impl IsA<IoctlClient>) -> bool {
        unsafe {
            from_glib(ffi::umockdev_ioctl_base_handle_ioctl(self.as_ref().to_glib_none().0, client.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "umockdev_ioctl_base_handle_read")]
    fn handle_read(&self, client: &impl IsA<IoctlClient>) -> bool {
        unsafe {
            from_glib(ffi::umockdev_ioctl_base_handle_read(self.as_ref().to_glib_none().0, client.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "umockdev_ioctl_base_handle_write")]
    fn handle_write(&self, client: &impl IsA<IoctlClient>) -> bool {
        unsafe {
            from_glib(ffi::umockdev_ioctl_base_handle_write(self.as_ref().to_glib_none().0, client.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "client-connected")]
    fn connect_client_connected<F: Fn(&Self, &IoctlClient) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn client_connected_trampoline<P: IsA<IoctlBase>, F: Fn(&P, &IoctlClient) + 'static>(this: *mut ffi::UMockdevIoctlBase, client: *mut ffi::UMockdevIoctlClient, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(IoctlBase::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(client))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"client-connected\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(client_connected_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "client-vanished")]
    fn connect_client_vanished<F: Fn(&Self, &IoctlClient) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn client_vanished_trampoline<P: IsA<IoctlBase>, F: Fn(&P, &IoctlClient) + 'static>(this: *mut ffi::UMockdevIoctlBase, client: *mut ffi::UMockdevIoctlClient, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(IoctlBase::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(client))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"client-vanished\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(client_vanished_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<IoctlBase>> IoctlBaseExt for O {}
