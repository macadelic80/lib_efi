//! Graphics Output Protocol
//!
//! Provides means to configure graphics hardware and get access to
//! framebuffers. Replaces the old UGA interface from EFI with a
//! VGA-independent API.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xBB25CF6F,
    0xF1D4,
    0x11D2,
    0x9A,
    0x0C,
    &[0x00, 0x99, 0x27, 0x3F, 0xC1, 0xFD],
);

pub const SERIAL_IO_PROTOCOL_REVISION:u32 =     0x00010000u32;
pub const SERIAL_IO_PROTOCOL_REVISION1P1: u32  = 0x00010001u32;

pub type ParityType = u32;
pub const DEFAULT_PARITY: ParityType  = 0x00000000;
pub const NO_PARITY: ParityType  = 0x00000001;
pub const EVEN_PARITY: ParityType  = 0x00000002;
pub const ODD_PARITY: ParityType  = 0x00000003;
pub const MARK_PARITY: ParityType  = 0x00000004;
pub const SPACE_PARITY: ParityType  = 0x00000005;

pub type StopBitsType = u32;

pub const DEFAULT_STOP_BITS: StopBitsType = 0x00000000;
pub const ONE_STOP_BIT: StopBitsType = 0x00000001;
pub const ONE_FIVE_STOP_BITS: StopBitsType = 0x00000002;
pub const TWO_STOP_BTS: StopBitsType = 0x00000003;


pub const SERIAL_CLEAR_TO_SEND: u32 =                0x0010;
pub const SERIAL_DATA_SET_READY: u32 =               0x0020;
pub const SERIAL_RING_INDICATE: u32 =                0x0040;
pub const SERIAL_CARRIER_DETECT: u32 =               0x0080;
pub const SERIAL_REQUEST_TO_SEND: u32 =              0x0002;
pub const SERIAL_DATA_TERMINAL_READY: u32 =          0x0001;
pub const SERIAL_INPUT_BUFFER_EMPTY: u32 =           0x0100;
pub const SERIAL_OUTPUT_BUFFER_EMPTY: u32 =          0x0200;
pub const SERIAL_HARDWARE_LOOPBACK_ENABLE: u32 =     0x1000;
pub const SERIAL_SOFTWARE_LOOPBACK_ENABLE: u32 =     0x2000;
pub const SERIAL_HARDWARE_FLOW_CONTROL_ENABLE: u32 = 0x4000;


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Mode {
    ///A mask of the Control bits that the device supports. The device must always support the Input Buffer Empty control bit.
    pub control_mask: u32,
    ///If applicable, the number of microseconds to wait before timing out a Read or Write operation.
    pub time_out: u32,
    ///If applicable, the current baud rate setting of the device; otherwise, baud rate has the value of zero to indicate that device runs at the device’s designed speed.
    pub baud_rate: u64,
    ///The number of characters the device will buffer on input.
    pub receive_fifo_depth: u32,
    ///The number of data bits in each character.
    pub data_bits: u32,
    ///If applicable, this is the EFI_PARITY_TYPE that is computed or checked as each character is transmitted or received. If the device does not support parity the value is the default parity value.
    pub parity: u32,
    ///If applicable, the EFI_STOP_BITS_TYPE number of stop bits per character. If the device does not support stop bits the value is the default stop bit value.
    pub stop_bits: u32,
}


pub type SetAttribute = eficall! {fn(
    this: *mut Protocol,
    baud_rate: u64,
    receive_fifo_depth: u32,
    time_out: u32,
    parity: u32,
    data_bits: u32,
    stop_bits: u32,
) -> crate::base::Status};


pub type Reset = eficall! {fn(
    this: *mut Protocol,
) -> crate::base::Status};


pub type SetControl = eficall! {fn(
    this: *mut Protocol,
    control: u32
) -> crate::base::Status};

pub type GetControl = eficall! {fn(
    this: *mut Protocol,
    control: *mut u32
) -> crate::base::Status};

pub type Write = eficall! {fn(
    this: *mut Protocol,
    buffer_size: *mut usize,
    buffer: *mut core::ffi::c_void
) -> crate::base::Status};

pub type Read = eficall! {fn(
    this: *mut Protocol,
    buffer_size: *mut usize,
    buffer: *mut core::ffi::c_void
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub revision: u32,
    pub reset: Reset,
    pub set_attribute: SetAttribute,
    pub set_control: SetControl,
    pub get_control: GetControl,
    pub write: Write,
    pub read: Read,
    pub mode: *mut Mode,
}
