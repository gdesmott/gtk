// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::fmt;
use StateFlags;
use WidgetPath;

glib_wrapper! {
    pub struct StyleProvider(Interface<gtk_sys::GtkStyleProvider>);

    match fn {
        get_type => || gtk_sys::gtk_style_provider_get_type(),
    }
}

pub const NONE_STYLE_PROVIDER: Option<&StyleProvider> = None;

pub trait StyleProviderExt: 'static {
    fn get_style_property(
        &self,
        path: &WidgetPath,
        state: StateFlags,
        pspec: &glib::ParamSpec,
    ) -> Option<glib::Value>;
}

impl<O: IsA<StyleProvider>> StyleProviderExt for O {
    fn get_style_property(
        &self,
        path: &WidgetPath,
        state: StateFlags,
        pspec: &glib::ParamSpec,
    ) -> Option<glib::Value> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret = from_glib(gtk_sys::gtk_style_provider_get_style_property(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                state.to_glib(),
                pspec.to_glib_none().0,
                value.to_glib_none_mut().0,
            ));
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }
}

impl fmt::Display for StyleProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StyleProvider")
    }
}
