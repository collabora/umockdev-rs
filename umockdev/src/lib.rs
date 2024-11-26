#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {};
}

pub use auto::*;
pub use umockdev_sys as ffi;
mod auto;

pub mod prelude {
    pub use crate::auto::traits::*;

    use super::*;
    use glib::prelude::*;

    // TODO: make sealed?
    pub trait TestbedManualExt: IsA<Testbed> + 'static {
        fn add_device(
            &self,
            subsystem: &str,
            name: &str,
            parent: Option<&str>,
            attributes: &[(&str, &str)],
            properties: &[(&str, &str)],
        ) -> Option<glib::GString> {
            let mut attributes_list = vec![];
            let mut properties_list = vec![];

            for (name, value) in attributes {
                attributes_list.push(*name);
                attributes_list.push(*value);
            }

            for (name, value) in properties {
                properties_list.push(*name);
                properties_list.push(*value);
            }

            self.add_devicev(subsystem, name, parent, &attributes_list, &properties_list)
        }
    }

    impl<O: IsA<Testbed>> TestbedManualExt for O {}
}
