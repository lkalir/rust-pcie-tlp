#![cfg_attr(not(test), no_std)]

mod address;
mod device_id;
mod headers;
mod packets;

pub use address::Address;
pub use device_id::DeviceID;
pub use headers::*;
pub use packets::*;

/// Data word size in bytes
pub const DWORD_LEN: usize = 4;

/// Maximum TLP payload size in bytes
pub const MAX_DATA_LEN: usize = 1024 * DWORD_LEN;

/// Maximum size of a TLP with data in bytes
pub const MAX_TLP_BUFFER: usize = 4 * DWORD_LEN + MAX_DATA_LEN * DWORD_LEN;
