// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,IoctlData};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "UMockdevIoctlClient")]
    pub struct IoctlClient(Object<ffi::UMockdevIoctlClient, ffi::UMockdevIoctlClientClass>);

    match fn {
        type_ => || ffi::umockdev_ioctl_client_get_type(),
    }
}

impl IoctlClient {
        pub const NONE: Option<&'static IoctlClient> = None;
    
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::IoctlClient>> Sealed for T {}
}

pub trait IoctlClientExt: IsA<IoctlClient> + sealed::Sealed + 'static {
    #[doc(alias = "umockdev_ioctl_client_get_devnode")]
    #[doc(alias = "get_devnode")]
    fn devnode(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::umockdev_ioctl_client_get_devnode(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "umockdev_ioctl_client_get_request")]
    #[doc(alias = "get_request")]
    fn request(&self) -> libc::c_ulong {
        unsafe {
            ffi::umockdev_ioctl_client_get_request(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "umockdev_ioctl_client_get_arg")]
    #[doc(alias = "get_arg")]
    fn arg(&self) -> IoctlData {
        unsafe {
            from_glib_none(ffi::umockdev_ioctl_client_get_arg(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "umockdev_ioctl_client_get_connected")]
    #[doc(alias = "get_connected")]
    fn is_connected(&self) -> bool {
        unsafe {
            from_glib(ffi::umockdev_ioctl_client_get_connected(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "umockdev_ioctl_client_execute")]
    fn execute(&self) -> Result<(i32, i32), glib::Error> {
        unsafe {
            let mut errno_ = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let ret = ffi::umockdev_ioctl_client_execute(self.as_ref().to_glib_none().0, errno_.as_mut_ptr(), &mut error);
            if error.is_null() { Ok((ret, errno_.assume_init())) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "umockdev_ioctl_client_complete")]
    fn complete(&self, res: libc::c_long, errno_: i32) {
        unsafe {
            ffi::umockdev_ioctl_client_complete(self.as_ref().to_glib_none().0, res, errno_);
        }
    }

    #[doc(alias = "umockdev_ioctl_client_abort")]
    fn abort(&self) {
        unsafe {
            ffi::umockdev_ioctl_client_abort(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "devnode")]
    fn connect_devnode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_devnode_trampoline<P: IsA<IoctlClient>, F: Fn(&P) + 'static>(this: *mut ffi::UMockdevIoctlClient, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(IoctlClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::devnode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_devnode_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "request")]
    fn connect_request_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_request_trampoline<P: IsA<IoctlClient>, F: Fn(&P) + 'static>(this: *mut ffi::UMockdevIoctlClient, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(IoctlClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_request_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "arg")]
    fn connect_arg_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_arg_trampoline<P: IsA<IoctlClient>, F: Fn(&P) + 'static>(this: *mut ffi::UMockdevIoctlClient, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(IoctlClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::arg\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_arg_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "connected")]
    fn connect_connected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_connected_trampoline<P: IsA<IoctlClient>, F: Fn(&P) + 'static>(this: *mut ffi::UMockdevIoctlClient, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(IoctlClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::connected\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_connected_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<IoctlClient>> IoctlClientExt for O {}
