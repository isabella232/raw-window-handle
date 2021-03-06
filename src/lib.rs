//! Interoperability library for Rust Windowing applications.
//!
//! This library provides standard types for accessing a window's platform-specific raw window
//! handle. This does not provide any utilities for creating and managing windows; instead, it
//! provides a common interface that window creation libraries (e.g. Winit, SDL) can use to easily
//! talk with graphics libraries (e.g. gfx-hal).
//!
//! ## Platform handle initialization
//!
//! Each platform handle struct is purposefully non-exhaustive, so that additional fields may be
//! added without breaking backwards compatibility. Each struct provides an `empty` method that may
//! be used along with the struct update syntax to construct it. See each specific struct for
//! examples.
//!
#![cfg_attr(feature = "nightly-docs", feature(doc_cfg))]
#![no_std]

#[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "android")))]
#[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "android"))]
pub mod android;
#[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "ios")))]
#[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "ios"))]
pub mod ios;
#[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "macos")))]
#[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "macos"))]
pub mod macos;
#[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "redox")))]
#[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "redox"))]
pub mod redox;
#[cfg_attr(
    feature = "nightly-docs",
    doc(cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "solaris"
    )))
)]
#[cfg_attr(
    not(feature = "nightly-docs"),
    cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "solaris"
    ))
)]
pub mod unix;
#[cfg_attr(feature = "nightly-docs", doc(cfg(target_arch = "wasm32")))]
#[cfg_attr(not(feature = "nightly-docs"), cfg(target_arch = "wasm32"))]
pub mod web;
#[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "windows")))]
#[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "windows"))]
pub mod windows;

mod platform {
    #[cfg(target_os = "android")]
    pub use crate::android::*;
    #[cfg(target_os = "macos")]
    pub use crate::macos::*;
    #[cfg(target_os = "redox")]
    pub use crate::redox::*;
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "solaris",
    ))]
    pub use crate::unix::*;
    #[cfg(target_os = "windows")]
    pub use crate::windows::*;
    // mod platform;
    #[cfg(target_os = "ios")]
    pub use crate::ios::*;
    #[cfg(target_arch = "wasm32")]
    pub use crate::web::*;
}

/// Window that wraps around a raw window handle.
///
/// # Safety guarantees
///
/// Users can safely assume that non-`null`/`0` fields are valid handles, and it is up to the
/// implementer of this trait to ensure that condition is upheld.
///
/// Despite that qualification, implementers should still make a best-effort attempt to fill in all
/// available fields. If an implementation doesn't, and a downstream user needs the field, it should
/// try to derive the field from other fields the implementer *does* provide via whatever methods the
/// platform provides.
///
/// The exact handles returned by `raw_window_handle` must remain consistent between multiple calls
/// to `raw_window_handle` as long as not indicated otherwise by platform specific events.
pub unsafe trait HasRawWindowHandle {
    fn raw_window_handle(&self) -> RawWindowHandle;
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawWindowHandle {
    #[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "ios")))]
    #[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "ios"))]
    IOS(ios::IOSHandle),

    #[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "macos")))]
    #[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "macos"))]
    MacOS(macos::MacOSHandle),

    #[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "redox")))]
    #[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "redox"))]
    Redox(redox::RedoxHandle),

    #[cfg_attr(
        feature = "nightly-docs",
        doc(cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "solaris"
        )))
    )]
    #[cfg_attr(
        not(feature = "nightly-docs"),
        cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "solaris"
        ))
    )]
    Xlib(unix::XlibHandle),

    #[cfg_attr(
        feature = "nightly-docs",
        doc(cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "solaris"
        )))
    )]
    #[cfg_attr(
        not(feature = "nightly-docs"),
        cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "solaris"
        ))
    )]
    Xcb(unix::XcbHandle),

    #[cfg_attr(
        feature = "nightly-docs",
        doc(cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        )))
    )]
    #[cfg_attr(
        not(feature = "nightly-docs"),
        cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))
    )]
    Wayland(unix::WaylandHandle),

    #[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "windows")))]
    #[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "windows"))]
    Windows(windows::WindowsHandle),

    #[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "windows")))]
    #[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "windows"))]
    WinRT(windows::WinRTHandle),

    #[cfg_attr(feature = "nightly-docs", doc(cfg(target_arch = "wasm32")))]
    #[cfg_attr(not(feature = "nightly-docs"), cfg(target_arch = "wasm32"))]
    Web(web::WebHandle),

    #[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "android")))]
    #[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "android"))]
    Android(android::AndroidHandle),
}

/// This wraps a [`RawWindowHandle`] to give it a [`HasRawWindowHandle`] impl.
///
/// The `HasRawWindowHandle` trait must be an `unsafe` trait because *other*
/// unsafe code is going to rely on it to provide accurate window handle info.
/// Since `RawWindowHandle` is an enum and enum fields are public, anyone could
/// make any random `RawWindowHandle` value in safe code.
///
/// The solution is that you assert that you're trusting a particular handle
/// value by (unsafely) placing it within this wrapper struct.
pub struct TrustedWindowHandle {
    raw: RawWindowHandle,
}
impl TrustedWindowHandle {
    /// Assert that the `RawWindowHandle` value can be trusted.
    ///
    /// ## Safety
    /// If the value violates any of the safety outlines given in the
    /// [`HasRawWindowHandle`] trait this can lead to UB.
    pub const unsafe fn new(raw: RawWindowHandle) -> Self {
        Self { raw }
    }
}
unsafe impl HasRawWindowHandle for TrustedWindowHandle {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.raw
    }
}
