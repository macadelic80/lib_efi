//! File Protocol
//!
//! Provides an interface to interact with both files and directories. This protocol is typically
//! obtained via an EFI_SIMPLE_FILE_SYSTEM protocol or via another EFI_FILE_PROTOCOL.

pub const REVISION: u64 = 0x0000_0000_0001_0000u64;
pub const REVISION2: u64 = 0x0000_0000_0002_0000u64;
pub const LATEST_REVISION: u64 = REVISION2;

pub const MODE_READ: u64 = 0x0000000000000001u64;
pub const MODE_WRITE: u64 = 0x0000000000000002u64;
pub const MODE_CREATE: u64 = 0x8000000000000000u64;

pub const READ_ONLY: u64 = 0x0000000000000001u64;
pub const HIDDEN: u64 = 0x0000000000000002u64;
pub const SYSTEM: u64 = 0x0000000000000004u64;
pub const RESERVED: u64 = 0x0000000000000008u64;
pub const DIRECTORY: u64 = 0x0000000000000010u64;
pub const ARCHIVE: u64 = 0x0000000000000020u64;
pub const VALID_ATTR: u64 = 0x0000000000000037u64;

pub const INFO_ID: crate::base::Guid = crate::base::Guid::from_fields(
    0x09576e92,
    0x6d3f,
    0x11d2,
    0x8e,
    0x39,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const SYSTEM_INFO_ID: crate::base::Guid = crate::base::Guid::from_fields(
    0x09576e93,
    0x6d3f,
    0x11d2,
    0x8e,
    0x39,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const SYSTEM_VOLUME_LABEL_ID: crate::base::Guid = crate::base::Guid::from_fields(
    0xdb47d7d3,
    0xfe81,
    0x11d3,
    0x9a,
    0x35,
    &[0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IoToken {
    pub event: crate::base::Event,
    pub status: crate::base::Status,
    pub buffer_size: usize,
    pub buffer: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
///The EFI_FILE_INFO data structure supports EFI_FILE_PROTOCOL.GetInfo() and EFI_FILE_PROTOCOL.SetInfo() requests. In the case of SetInfo(), the following additional rules apply:

///On directories, the file size is determined by the contents of the directory and cannot be changed by setting FileSize. On directories, FileSize is ignored during a SetInfo().

///The PhysicalSize is determined by the FileSize and cannot be changed. This value is ignored during a SetInfo() request.

///The EFI_FILE_DIRECTORY attribute bit cannot be changed. It must match the file’s actual type.

///A value of zero in CreateTime, LastAccess, or ModificationTime causes the fields to be ignored (and not updated).
pub struct Info<const N: usize = 0> {
    ///Size of the EFI_FILE_INFO structure, including the Null-terminated FileName string.
    pub size: u64,
    ///The size of the file in bytes.
    pub file_size: u64,
    ///The amount of physical space the file consumes on the file system volume.
    pub physical_size: u64,
    ///The time the file was created.
    pub create_time: crate::system::Time,
    ///The time when the file was last accessed.
    pub last_access_time: crate::system::Time,
    ///The time when the file’s contents were last modified.
    pub modification_time: crate::system::Time,
    ///The attribute bits for the file.
    pub attribute: u64,
    ///The Null-terminated name of the file. For a root directory, the name is an empty string.
    pub file_name: [crate::base::Char16; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
///The EFI_FILE_SYSTEM_INFO data structure is an information structure that can be obtained on the root directory file handle. The root directory file handle is the file handle first obtained on the initial call to the EFI_BOOT_SERVICES.HandleProtocol() function to open the file system interface. All of the fields are read-only except for VolumeLabel. The system volume’s VolumeLabel can be created or modified by calling EFI_FILE_PROTOCOL.SetInfo() with an updated VolumeLabel field.
pub struct SystemInfo<const N: usize = 0> {
    ///Size of the EFI_FILE_SYSTEM_INFO structure, including he Null-terminated VolumeLabel string.
    pub size: u64,
    ///TRUE if the volume only supports read access.
    pub read_only: crate::base::Boolean,
    ///The number of bytes managed by the file system.
    pub volume_size: u64,
    ///The number of available bytes for use by the file system.
    pub free_space: u64,
    ///The nominal block size by which files are typically grown.
    pub block_size: u32,
    ///The Null-terminated string that is the volume’s label.
    pub volume_label: [crate::base::Char16; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
///The EFI_FILE_SYSTEM_VOLUME_LABEL data structure is an information structure that can be obtained on the root directory file handle. The root directory file handle is the file handle first obtained on the initial call to the EFI_BOOT_SERVICES.HandleProtocol() function to open the file system interface. The system volume’s VolumeLabel can be created or modified by calling EFI_FILE_PROTOCOL.SetInfo() with an updated VolumeLabel field.
pub struct SystemVolumeLabel<const N: usize = 0> {
    ///The Null-terminated string that is the volume’s label.
    pub volume_label: [crate::base::Char16; N],
}

pub type ProtocolOpen = eficall! {fn(
    *mut Protocol,
    *mut *mut Protocol,
    *mut crate::base::Char16,
    u64,
    u64,
) -> crate::base::Status};

pub type ProtocolClose = eficall! {fn(
    *mut Protocol,
) -> crate::base::Status};

pub type ProtocolDelete = eficall! {fn(
    *mut Protocol,
) -> crate::base::Status};

pub type ProtocolRead = eficall! {fn(
    *mut Protocol,
    *mut usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolWrite = eficall! {fn(
    *mut Protocol,
    *mut usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolGetPosition = eficall! {fn(
    *mut Protocol,
    *mut u64,
) -> crate::base::Status};

pub type ProtocolSetPosition = eficall! {fn(
    *mut Protocol,
    u64,
) -> crate::base::Status};

pub type ProtocolGetInfo = eficall! {fn(
    *mut Protocol,
    *mut crate::base::Guid,
    *mut usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolSetInfo = eficall! {fn(
    *mut Protocol,
    *mut crate::base::Guid,
    usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolFlush = eficall! {fn(
    *mut Protocol,
) -> crate::base::Status};

pub type ProtocolOpenEx = eficall! {fn(
    *mut Protocol,
    *mut *mut Protocol,
    *mut crate::base::Char16,
    u64,
    u64,
    *mut IoToken,
) -> crate::base::Status};

pub type ProtocolReadEx = eficall! {fn(
    *mut Protocol,
    *mut IoToken,
) -> crate::base::Status};

pub type ProtocolWriteEx = eficall! {fn(
    *mut Protocol,
    *mut IoToken,
) -> crate::base::Status};

pub type ProtocolFlushEx = eficall! {fn(
    *mut Protocol,
    *mut IoToken,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub open: ProtocolOpen,
    pub close: ProtocolClose,
    pub delete: ProtocolDelete,
    pub read: ProtocolRead,
    pub write: ProtocolWrite,
    pub get_position: ProtocolGetPosition,
    pub set_position: ProtocolSetPosition,
    pub get_info: ProtocolGetInfo,
    pub set_info: ProtocolSetInfo,
    pub flush: ProtocolFlush,
    pub open_ex: ProtocolOpenEx,
    pub read_ex: ProtocolReadEx,
    pub write_ex: ProtocolWriteEx,
    pub flush_ex: ProtocolFlushEx,
}
