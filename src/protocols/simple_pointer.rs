//! Absolute Pointer Protocol
//!
//! Provides a simple method for accessing absolute pointer devices. This
//! includes devices such as touch screens and digitizers. The Absolute Pointer
//! Protocol allows information about a pointer device to be retrieved. The
//! protocol is attached to the device handle of an absolute pointer device,
//! and can be used for input from the user in the preboot environment.
//!
//! Supported devices may return 1, 2, or 3 axis of information. The Z axis may
//! optionally be used to return pressure data measurements derived from user
//! pen force.
//!
//! All supported devices must support a touch-active status. Supported devices
//! may optionally support a second input button, for example a pen
//! side-button.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x31878c87,
    0xb75,
    0x11d5,
    0x9a,
    0x4f,
    &[0x00, 0x90,0x27,0x3f,0xc1,0x4d],
);
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Mode {
    ///The resolution of the pointer device on the x-axis in counts/mm. If 0, then the pointer device does not support an x-axis.
    pub resolution_x: u64,
    ///The resolution of the pointer device on the y-axis in counts/mm. If 0, then the pointer device does not support a y-axis.
    pub resolution_y: u64,
    ///The resolution of the pointer device on the z-axis in counts/mm. If 0, then the pointer device does not support a z-axis.
    pub resolution_z: u64,
    ///TRUE if a left button is present on the pointer device. Otherwise FALSE.
    pub left_button: bool,
    ///TRUE if a right button is present on the pointer device. Otherwise FALSE.
    pub right_button: bool,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct State {
    pub relative_movement_x: u32,
    pub relative_movement_y: u32,
    pub relative_movement_z: u32,
    pub left_button: crate::base::Boolean,
    pub right_button: crate::base::Boolean,
}

pub type Reset = eficall! {fn(
    this: *mut Protocol,
    extended_verification: crate::base::Boolean,
) -> crate::base::Status};

pub type GetState = eficall! {fn(
    this: *mut Protocol,
    state: *mut State,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub reset: Reset,
    pub get_state: GetState,
    pub wait_for_input: crate::efi::Event,
    pub mode: *mut Mode,
}
