//! # DBus interface proxy for: `org.a11y.atspi.DeviceEventController`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `DeviceEventController.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use serde::{Deserialize, Serialize};
use zbus::{dbus_proxy, zvariant::Type};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
pub enum EventType {
    KeyPressed,
    KeyReleased,
    ButtonPressed,
    ButtonReleased,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
pub enum KeySynthType {
    Press,
    Release,
    Pressrelease,
    Sym,
    String,
    Lockmodifiers,
    Unlockmodifiers,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct DeviceEvent<'a> {
    pub event_type: EventType,
    pub id: u32,
    pub hw_code: u32,
    pub modifiers: u32,
    pub timestamp: u32,
    pub event_string: &'a str,
    pub is_text: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct EventListenerMode {
    /// Whether events are delivered synchronously, before the currently focused application sees them.
    /// If `false`, events may be delivered asynchronously, which means in some
    /// cases they may already have been delivered to the
    /// application before the AT client receives the notification.
    pub synchronous: bool,
    /// Whether events may be consumed by the AT client.
    /// Requires [`EventListenerMode::synchronous`] to be set to `true`.
    pub preemptive: bool,
    /// If `true`, indicates that events are received not from the application toolkit layer,
    /// but from the device driver or windowing system subsystem.
    pub global: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct KeyDefinition<'a> {
    pub keycode: i32,
    pub keysym: i32,
    pub keystring: &'a str,
    pub unused: i32,
}

#[dbus_proxy(interface = "org.a11y.atspi.DeviceEventController", assume_defaults = true)]
trait DeviceEventController {
    /// DeregisterDeviceEventListener method
    fn deregister_device_event_listener(
        &self,
        listener: &zbus::zvariant::ObjectPath<'_>,
        types: EventType,
    ) -> zbus::Result<()>;

    /// DeregisterKeystrokeListener method
    fn deregister_keystroke_listener(
        &self,
        listener: &zbus::zvariant::ObjectPath<'_>,
        keys: &[KeyDefinition<'_>],
        mask: u32,
        type_: EventType,
    ) -> zbus::Result<()>;

    /// GenerateKeyboardEvent method
    fn generate_keyboard_event(
        &self,
        keycode: i32,
        keystring: &str,
        type_: KeySynthType,
    ) -> zbus::Result<()>;

    /// GenerateMouseEvent method
    fn generate_mouse_event(&self, x: i32, y: i32, event_name: &str) -> zbus::Result<()>;

    /// NotifyListenersAsync method
    fn notify_listeners_async(
        &self,
        event: &DeviceEvent<'_>,
    ) -> zbus::Result<()>;

    /// NotifyListenersSync method
    fn notify_listeners_sync(
        &self,
        event: &DeviceEvent<'_>,
    ) -> zbus::Result<bool>;

    /// RegisterDeviceEventListener method
    fn register_device_event_listener(
        &self,
        listener: &zbus::zvariant::ObjectPath<'_>,
        types: EventType,
    ) -> zbus::Result<bool>;

    /// RegisterKeystrokeListener method
    fn register_keystroke_listener(
        &self,
        listener: &zbus::zvariant::ObjectPath<'_>,
        keys: &[KeyDefinition<'_>],
        mask: u32,
        type_: &[EventType],
        mode: &EventListenerMode,
    ) -> zbus::Result<bool>;
}
