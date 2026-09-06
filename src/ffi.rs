// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

//! FFI bindings for the Rust library
//!
//! This module provides a C-compatible interface for the Rust library, allowing
//! C programs to use the library's functionality.

#![cfg(all(
    feature = "ffi",
    any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86")
))]

use crate::CrcAlgorithm;
use crate::CrcParams;
use crate::{get_calculator_target, Digest};
use core::ffi::c_char;
use core::ffi::CStr;
use core::slice;
use hashbrown::{HashMap, HashSet};
use spin::{Mutex, Once};

#[cfg(feature = "alloc")]
extern crate alloc;

static STRING_CACHE: Once<Mutex<HashSet<&'static str>>> = Once::new();

/// Error codes for FFI operations
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrcFastError {
    /// Operation completed successfully
    Success = 0,
    /// Null pointer was passed where non-null required
    NullPointer = 2,
    /// Invalid key count for CRC parameters
    InvalidKeyCount = 3,
    /// Unsupported CRC width (must be 32 or 64)
    UnsupportedWidth = 4,
    /// Invalid UTF-8 string
    InvalidUtf8 = 5,
    /// File I/O error
    IoError = 6,
    /// Internal string conversion error
    StringConversionError = 7,
}

impl CrcFastError {
    /// Returns a static string describing the error
    fn message(&self) -> &'static str {
        match self {
            CrcFastError::Success => "Operation completed successfully",
            CrcFastError::NullPointer => "Null pointer was passed where non-null required",
            CrcFastError::InvalidKeyCount => "Invalid key count for CRC parameters",
            CrcFastError::UnsupportedWidth => "Unsupported CRC width (must be 32 or 64)",
            CrcFastError::InvalidUtf8 => "Invalid UTF-8 string",
            CrcFastError::IoError => "File I/O error",
            CrcFastError::StringConversionError => "Internal string conversion error",
        }
    }
}

// Thread-local storage for the last error that occurred
thread_local! {
    static LAST_ERROR: core::cell::Cell<CrcFastError> = const { core::cell::Cell::new(CrcFastError::Success) };
}

/// Sets the thread-local last error
fn set_last_error(error: CrcFastError) {
    LAST_ERROR.with(|e| e.set(error));
}

/// Clears the thread-local last error (sets it to Success)
fn clear_last_error() {
    LAST_ERROR.with(|e| e.set(CrcFastError::Success));
}

// Global storage for stable key pointers to ensure they remain valid across FFI boundary
static STABLE_KEY_STORAGE: Once<Mutex<HashMap<u64, Box<[u64]>>>> = Once::new();

/// Creates a stable pointer to the keys for FFI usage.
/// The keys are stored in global memory to ensure the pointer remains valid.
/// Returns (pointer, count) on success, or (null, 0) on error.
fn create_stable_key_pointer(keys: &crate::CrcKeysStorage) -> (*const u64, u32) {
    let storage = STABLE_KEY_STORAGE.call_once(|| Mutex::new(HashMap::new()));

    // Create a unique hash for this key set to avoid duplicates
    let key_hash = match keys {
        crate::CrcKeysStorage::KeysFold256(keys) => {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use core::hash::{Hash, Hasher};
            keys.hash(&mut hasher);
            hasher.finish()
        }
        crate::CrcKeysStorage::KeysFutureTest(keys) => {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use core::hash::{Hash, Hasher};
            keys.hash(&mut hasher);
            hasher.finish()
        }
    };

    let mut storage_map = storage.lock();

    // Check if we already have this key set stored
    if let Some(stored_keys) = storage_map.get(&key_hash) {
        return (stored_keys.as_ptr(), stored_keys.len() as u32);
    }

    // Store the keys in stable memory
    let key_vec: Vec<u64> = match keys {
        crate::CrcKeysStorage::KeysFold256(keys) => keys.to_vec(),
        crate::CrcKeysStorage::KeysFutureTest(keys) => keys.to_vec(),
    };

    let boxed_keys = key_vec.into_boxed_slice();
    let count = boxed_keys.len() as u32;
    // Use entry to avoid Stacked Borrows invalidation: ptr must be derived from
    // the hashbrown allocation *after* insertion, not from the pre-move Box.
    let ptr = storage_map.entry(key_hash).or_insert(boxed_keys).as_ptr();

    (ptr, count)
}

/// A handle to the Digest object
#[repr(C)]
pub struct CrcFastDigestHandle(*mut Digest);

/// The supported CRC algorithms
///
/// Mirrors `CrcAlgorithm` by hand: the FFI enum must stay a plain `#[repr(C)]`
/// definition for cbindgen, and `macro_rules!` cannot expand to enum variants.
/// Both conversions are generated from one list (`impl_algorithm_conversions!`
/// below), so only the declarations mirror.
/// `CrcCustom` works with any supported width (5, 8, 16, 31, 32, 64).
#[repr(C)]
#[derive(Clone, Copy)]
pub enum CrcFastAlgorithm {
    CrcCustom,
    Crc16Arc,
    Crc16Cdma2000,
    Crc16Cms,
    Crc16Dds110,
    Crc16DectR,
    Crc16DectX,
    Crc16Dnp,
    Crc16En13757,
    Crc16Genibus,
    Crc16Gsm,
    Crc16Ibm3740,
    Crc16IbmSdlc,
    Crc16IsoIec144433A,
    Crc16Kermit,
    Crc16Lj1200,
    Crc16M17,
    Crc16MaximDow,
    Crc16Mcrf4xx,
    Crc16Modbus,
    Crc16Nrsc5,
    Crc16OpensafetyA,
    Crc16OpensafetyB,
    Crc16Profibus,
    Crc16Riello,
    Crc16SpiFujitsu,
    Crc16T10Dif,
    Crc16Teledisk,
    Crc16Tms37157,
    Crc16Umts,
    Crc16Usb,
    Crc16Xmodem,
    Crc5Usb,
    Crc5EpcC1G2,
    Crc5G704,
    Crc8Smbus,
    Crc8I4321,
    Crc8Rohc,
    Crc8GsmA,
    Crc8MifareMad,
    Crc8ICode,
    Crc8Hitag,
    Crc8SaeJ1850,
    Crc8Tech3250,
    Crc8Opensafety,
    Crc8Autosar,
    Crc8MaximDow,
    Crc8Nrsc5,
    Crc8Darc,
    Crc8GsmB,
    Crc8Lte,
    Crc8Wcdma,
    Crc8Cdma2000,
    Crc8Bluetooth,
    Crc8DvbS2,
    Crc31Philips,
    Crc32Aixm,
    Crc32Autosar,
    Crc32Base91D,
    Crc32Bzip2,
    Crc32CdRomEdc,
    Crc32Cksum,
    Crc32Iscsi,
    Crc32IsoHdlc,
    Crc32Jamcrc,
    Crc32Mef,
    Crc32Mpeg2,
    Crc32Xfer,
    Crc64Ecma182,
    Crc64GoIso,
    Crc64Ms,
    Crc64Nvme,
    Crc64Redis,
    Crc64We,
    Crc64Xz,
}

/// Generates both algorithm-enum conversions from one variant list so they
/// cannot drift. Item position: expands to two `From` impls with concrete
/// match arms (no nested macro calls).
macro_rules! impl_algorithm_conversions {
    ([$($v:ident),* $(,)?]) => {
        impl From<CrcFastAlgorithm> for CrcAlgorithm {
            fn from(value: CrcFastAlgorithm) -> Self {
                match value {
                    $(CrcFastAlgorithm::$v => CrcAlgorithm::$v,)*
                }
            }
        }

        impl From<CrcAlgorithm> for CrcFastAlgorithm {
            fn from(value: CrcAlgorithm) -> Self {
                match value {
                    $(CrcAlgorithm::$v => CrcFastAlgorithm::$v,)*
                }
            }
        }
    };
}

impl_algorithm_conversions!([
    CrcCustom,
    Crc16Arc,
    Crc16Cdma2000,
    Crc16Cms,
    Crc16Dds110,
    Crc16DectR,
    Crc16DectX,
    Crc16Dnp,
    Crc16En13757,
    Crc16Genibus,
    Crc16Gsm,
    Crc16Ibm3740,
    Crc16IbmSdlc,
    Crc16IsoIec144433A,
    Crc16Kermit,
    Crc16Lj1200,
    Crc16M17,
    Crc16MaximDow,
    Crc16Mcrf4xx,
    Crc16Modbus,
    Crc16Nrsc5,
    Crc16OpensafetyA,
    Crc16OpensafetyB,
    Crc16Profibus,
    Crc16Riello,
    Crc16SpiFujitsu,
    Crc16T10Dif,
    Crc16Teledisk,
    Crc16Tms37157,
    Crc16Umts,
    Crc16Usb,
    Crc16Xmodem,
    Crc5Usb,
    Crc5EpcC1G2,
    Crc5G704,
    Crc8Smbus,
    Crc8I4321,
    Crc8Rohc,
    Crc8GsmA,
    Crc8MifareMad,
    Crc8ICode,
    Crc8Hitag,
    Crc8SaeJ1850,
    Crc8Tech3250,
    Crc8Opensafety,
    Crc8Autosar,
    Crc8MaximDow,
    Crc8Nrsc5,
    Crc8Darc,
    Crc8GsmB,
    Crc8Lte,
    Crc8Wcdma,
    Crc8Cdma2000,
    Crc8Bluetooth,
    Crc8DvbS2,
    Crc31Philips,
    Crc32Aixm,
    Crc32Autosar,
    Crc32Base91D,
    Crc32Bzip2,
    Crc32CdRomEdc,
    Crc32Cksum,
    Crc32Iscsi,
    Crc32IsoHdlc,
    Crc32Jamcrc,
    Crc32Mef,
    Crc32Mpeg2,
    Crc32Xfer,
    Crc64Ecma182,
    Crc64GoIso,
    Crc64Ms,
    Crc64Nvme,
    Crc64Redis,
    Crc64We,
    Crc64Xz,
]);

/// Gets the last error that occurred in the current thread
/// Returns CrcFastError::Success if no error has occurred
#[no_mangle]
pub extern "C" fn crc_fast_get_last_error() -> CrcFastError {
    LAST_ERROR.with(|e| e.get())
}

/// Clears the last error for the current thread
#[no_mangle]
pub extern "C" fn crc_fast_clear_error() {
    clear_last_error();
}

/// Gets a human-readable error message for the given error code
/// Returns a pointer to a static string (do not free)
#[no_mangle]
pub extern "C" fn crc_fast_error_message(error: CrcFastError) -> *const c_char {
    let message = error.message();
    // These are static strings, so we can safely return them as C strings
    // The strings are guaranteed to be valid UTF-8 and null-terminated
    match alloc::ffi::CString::new(message) {
        Ok(c_str) => {
            // Leak the string so it remains valid for the lifetime of the program
            // This is safe because error messages are static and small
            Box::leak(Box::new(c_str)).as_ptr()
        }
        Err(_) => core::ptr::null(),
    }
}

/// Custom CRC parameters
#[repr(C)]
pub struct CrcFastParams {
    pub algorithm: CrcFastAlgorithm,
    pub width: u8,
    pub poly: u64,
    pub init: u64,
    pub refin: bool,
    pub refout: bool,
    pub xorout: u64,
    pub check: u64,
    pub key_count: u32,
    pub keys: *const u64,
}

/// Fallible conversion from FFI struct to internal struct
/// Returns None if the parameters are invalid (unsupported key count)
fn try_params_from_ffi(value: &CrcFastParams) -> Option<CrcParams> {
    // Validate key pointer
    if value.keys.is_null() {
        return None;
    }

    // Convert C array back to appropriate CrcKeysStorage
    let keys = unsafe { core::slice::from_raw_parts(value.keys, value.key_count as usize) };

    let storage = match value.key_count {
        23 => match keys.try_into() {
            Ok(arr) => crate::CrcKeysStorage::from_keys_fold_256(arr),
            Err(_) => return None,
        },
        25 => match keys.try_into() {
            Ok(arr) => crate::CrcKeysStorage::from_keys_fold_future_test(arr),
            Err(_) => return None,
        },
        _ => return None, // Unsupported key count
    };

    // For reflected CRCs, bit-reverse the init value for the SIMD algorithm
    let init_algorithm = crate::structs::reflected_init(value.refin, value.width, value.init);

    Some(CrcParams {
        algorithm: value.algorithm.into(),
        name: "custom", // C interface doesn't need the name field
        width: value.width,
        poly: value.poly,
        init: value.init,
        init_algorithm,
        refin: value.refin,
        refout: value.refout,
        xorout: value.xorout,
        check: value.check,
        keys: storage,
    })
}

// Convert from FFI struct to internal struct (legacy, may panic)
// For backwards compatibility, but prefer try_params_from_ffi
impl From<CrcFastParams> for CrcParams {
    fn from(value: CrcFastParams) -> Self {
        try_params_from_ffi(&value)
            .unwrap_or_else(|| unsafe { core::hint::unreachable_unchecked() })
    }
}

// Convert from internal struct to FFI struct
impl From<CrcParams> for CrcFastParams {
    fn from(params: CrcParams) -> Self {
        // Create stable key pointer for FFI usage
        let (keys_ptr, key_count) = create_stable_key_pointer(&params.keys);

        CrcFastParams {
            algorithm: params.algorithm.into(),
            width: params.width,
            poly: params.poly,
            init: params.init,
            refin: params.refin,
            refout: params.refout,
            xorout: params.xorout,
            check: params.check,
            key_count,
            keys: keys_ptr,
        }
    }
}

/// Allocates a `Digest` on the heap and leaks a handle to it for FFI.
/// Single source for the allocate-wrap-leak tail shared by every
/// `crc_fast_digest_new*` constructor.
fn leak_digest_handle(digest: Digest) -> *mut CrcFastDigestHandle {
    let raw = Box::into_raw(Box::new(digest));
    Box::into_raw(Box::new(CrcFastDigestHandle(raw)))
}

/// Creates a new Digest to compute CRC checksums using algorithm
#[no_mangle]
pub extern "C" fn crc_fast_digest_new(algorithm: CrcFastAlgorithm) -> *mut CrcFastDigestHandle {
    clear_last_error();
    leak_digest_handle(Digest::new(algorithm.into()))
}

/// Creates a new Digest with a custom initial state
#[no_mangle]
pub extern "C" fn crc_fast_digest_new_with_init_state(
    algorithm: CrcFastAlgorithm,
    init_state: u64,
) -> *mut CrcFastDigestHandle {
    clear_last_error();
    leak_digest_handle(Digest::new_with_init_state(algorithm.into(), init_state))
}

/// Creates a new Digest to compute CRC checksums using custom parameters
/// Returns NULL if parameters are invalid (invalid key count or null pointer)
/// Call crc_fast_get_last_error() to get the specific error code
#[no_mangle]
pub extern "C" fn crc_fast_digest_new_with_params(
    params: CrcFastParams,
) -> *mut CrcFastDigestHandle {
    clear_last_error();
    match try_params_from_ffi(&params) {
        Some(crc_params) => leak_digest_handle(Digest::new_with_params(crc_params)),
        None => {
            // Set appropriate error based on the failure
            if params.keys.is_null() {
                set_last_error(CrcFastError::NullPointer);
            } else {
                set_last_error(CrcFastError::InvalidKeyCount);
            }
            core::ptr::null_mut()
        }
    }
}

/// Updates the Digest with data
#[no_mangle]
pub extern "C" fn crc_fast_digest_update(
    handle: *mut CrcFastDigestHandle,
    data: *const c_char,
    len: usize,
) {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return;
    }
    if data.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return;
    }

    clear_last_error();
    unsafe {
        let digest = &mut *(*handle).0;

        #[allow(clippy::unnecessary_cast)]
        let bytes = slice::from_raw_parts(data as *const u8, len);
        digest.update(bytes);
    }
}

/// Calculates the CRC checksum for data that's been written to the Digest
/// Returns 0 on error (e.g. null handle)
#[no_mangle]
pub extern "C" fn crc_fast_digest_finalize(handle: *mut CrcFastDigestHandle) -> u64 {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }

    clear_last_error();
    unsafe {
        let digest = &*(*handle).0;
        digest.finalize()
    }
}

/// Free the Digest resources without finalizing
#[no_mangle]
pub extern "C" fn crc_fast_digest_free(handle: *mut CrcFastDigestHandle) {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return;
    }

    clear_last_error();
    unsafe {
        let handle = Box::from_raw(handle);
        let _ = Box::from_raw(handle.0); // This drops the digest
    }
}

/// Reset the Digest state
#[no_mangle]
pub extern "C" fn crc_fast_digest_reset(handle: *mut CrcFastDigestHandle) {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return;
    }

    clear_last_error();
    unsafe {
        let digest = &mut *(*handle).0;

        digest.reset();
    }
}

/// Finalize and reset the Digest in one operation
/// Returns 0 on error (e.g. null handle)
#[no_mangle]
pub extern "C" fn crc_fast_digest_finalize_reset(handle: *mut CrcFastDigestHandle) -> u64 {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }

    clear_last_error();
    unsafe {
        let digest = &mut *(*handle).0;

        digest.finalize_reset()
    }
}

/// Combine two Digest checksums
#[no_mangle]
pub extern "C" fn crc_fast_digest_combine(
    handle1: *mut CrcFastDigestHandle,
    handle2: *mut CrcFastDigestHandle,
) {
    if handle1.is_null() || handle2.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return;
    }

    clear_last_error();
    unsafe {
        let digest1 = &mut *(*handle1).0;
        let digest2 = &*(*handle2).0;
        digest1.combine(digest2);
    }
}

/// Gets the amount of data processed by the Digest so far
/// Returns 0 on error (e.g. null handle)
#[no_mangle]
pub extern "C" fn crc_fast_digest_get_amount(handle: *mut CrcFastDigestHandle) -> u64 {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }

    clear_last_error();
    unsafe {
        let digest = &*(*handle).0;
        digest.get_amount()
    }
}

/// Gets the current state of the Digest
/// Returns 0 on error (e.g. null handle)
#[no_mangle]
pub extern "C" fn crc_fast_digest_get_state(handle: *mut CrcFastDigestHandle) -> u64 {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }
    clear_last_error();
    unsafe {
        let digest = &*(*handle).0;
        digest.get_state()
    }
}

/// Helper method to calculate a CRC checksum directly for a string using algorithm
/// Returns 0 on error (e.g. null data pointer)
#[no_mangle]
pub extern "C" fn crc_fast_checksum(
    algorithm: CrcFastAlgorithm,
    data: *const c_char,
    len: usize,
) -> u64 {
    if data.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }
    clear_last_error();
    unsafe {
        #[allow(clippy::unnecessary_cast)]
        let bytes = slice::from_raw_parts(data as *const u8, len);
        crate::checksum(algorithm.into(), bytes)
    }
}

/// Helper method to calculate a CRC checksum directly for data using custom parameters
/// Returns 0 if parameters are invalid or data is null
/// Call crc_fast_get_last_error() to get the specific error code
#[no_mangle]
pub extern "C" fn crc_fast_checksum_with_params(
    params: CrcFastParams,
    data: *const c_char,
    len: usize,
) -> u64 {
    if data.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }
    match try_params_from_ffi(&params) {
        Some(crc_params) => {
            clear_last_error();
            unsafe {
                #[allow(clippy::unnecessary_cast)]
                let bytes = slice::from_raw_parts(data as *const u8, len);
                crate::checksum_with_params(crc_params, bytes)
            }
        }
        None => {
            if params.keys.is_null() {
                set_last_error(CrcFastError::NullPointer);
            } else {
                set_last_error(CrcFastError::InvalidKeyCount);
            }
            0
        }
    }
}

/// Helper method to just calculate a CRC checksum directly for a file using algorithm
/// Returns 0 if path is null or file I/O fails
/// Call crc_fast_get_last_error() to get the specific error code
#[no_mangle]
pub extern "C" fn crc_fast_checksum_file(
    algorithm: CrcFastAlgorithm,
    path_ptr: *const u8,
    path_len: usize,
) -> u64 {
    if path_ptr.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }

    unsafe {
        match crate::checksum_file(
            algorithm.into(),
            &convert_to_string(path_ptr, path_len),
            None,
        ) {
            Ok(result) => {
                clear_last_error();
                result
            }
            Err(_) => {
                set_last_error(CrcFastError::IoError);
                0
            }
        }
    }
}

/// Helper method to calculate a CRC checksum directly for a file using custom parameters
/// Returns 0 if parameters are invalid, path is null, or file I/O fails
/// Call crc_fast_get_last_error() to get the specific error code
#[no_mangle]
pub extern "C" fn crc_fast_checksum_file_with_params(
    params: CrcFastParams,
    path_ptr: *const u8,
    path_len: usize,
) -> u64 {
    if path_ptr.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }

    match try_params_from_ffi(&params) {
        Some(crc_params) => unsafe {
            match crate::checksum_file_with_params(
                crc_params,
                &convert_to_string(path_ptr, path_len),
                None,
            ) {
                Ok(result) => {
                    clear_last_error();
                    result
                }
                Err(_) => {
                    set_last_error(CrcFastError::IoError);
                    0
                }
            }
        },
        None => {
            if params.keys.is_null() {
                set_last_error(CrcFastError::NullPointer);
            } else {
                set_last_error(CrcFastError::InvalidKeyCount);
            }
            0
        }
    }
}

/// Combine two CRC checksums using algorithm
#[no_mangle]
pub extern "C" fn crc_fast_checksum_combine(
    algorithm: CrcFastAlgorithm,
    checksum1: u64,
    checksum2: u64,
    checksum2_len: u64,
) -> u64 {
    clear_last_error();
    crate::checksum_combine(algorithm.into(), checksum1, checksum2, checksum2_len)
}

/// Combine two CRC checksums using custom parameters
/// Returns 0 if parameters are invalid
/// Call crc_fast_get_last_error() to get the specific error code
#[no_mangle]
pub extern "C" fn crc_fast_checksum_combine_with_params(
    params: CrcFastParams,
    checksum1: u64,
    checksum2: u64,
    checksum2_len: u64,
) -> u64 {
    match try_params_from_ffi(&params) {
        Some(crc_params) => {
            clear_last_error();
            crate::checksum_combine_with_params(crc_params, checksum1, checksum2, checksum2_len)
        }
        None => {
            if params.keys.is_null() {
                set_last_error(CrcFastError::NullPointer);
            } else {
                set_last_error(CrcFastError::InvalidKeyCount);
            }
            0
        }
    }
}

/// Returns the custom CRC parameters for a given set of Rocksoft CRC parameters
/// If width is not 32 or 64, sets error to UnsupportedWidth
#[no_mangle]
pub extern "C" fn crc_fast_get_custom_params(
    name_ptr: *const c_char,
    width: u8,
    poly: u64,
    init: u64,
    reflected: bool,
    xorout: u64,
    check: u64,
) -> CrcFastParams {
    // Validate width
    if width != 32 && width != 64 {
        set_last_error(CrcFastError::UnsupportedWidth);
    } else {
        clear_last_error();
    }

    let name = if name_ptr.is_null() {
        "custom"
    } else {
        unsafe {
            match CStr::from_ptr(name_ptr).to_str() {
                Ok(s) => s,
                Err(_) => {
                    set_last_error(CrcFastError::InvalidUtf8);
                    "custom"
                }
            }
        }
    };

    // Get the custom params from the library
    let params = CrcParams::new(
        get_or_leak_string(name), // ✅ Use cached leak
        width,
        poly,
        init,
        reflected,
        xorout,
        check,
    );

    // Create stable key pointer for FFI usage
    let (keys_ptr, key_count) = create_stable_key_pointer(&params.keys);

    // Convert to FFI struct - use CrcCustom for all widths since CrcParams::new now uses it
    CrcFastParams {
        algorithm: CrcFastAlgorithm::CrcCustom,
        width: params.width,
        poly: params.poly,
        init: params.init,
        refin: params.refin,
        refout: params.refout,
        xorout: params.xorout,
        check: params.check,
        key_count,
        keys: keys_ptr,
    }
}

/// Gets the target build properties (CPU architecture and fine-tuning parameters) for this algorithm
/// Returns NULL if string conversion fails
/// Call crc_fast_get_last_error() to get the specific error code
#[no_mangle]
pub extern "C" fn crc_fast_get_calculator_target(algorithm: CrcFastAlgorithm) -> *const c_char {
    let target = get_calculator_target(algorithm.into());

    match alloc::ffi::CString::new(target) {
        Ok(s) => {
            clear_last_error();
            s.into_raw()
        }
        Err(_) => {
            set_last_error(CrcFastError::StringConversionError);
            core::ptr::null_mut()
        }
    }
}

/// Gets the version of this library
/// Returns a pointer to "unknown" if version string is invalid
#[no_mangle]
pub extern "C" fn crc_fast_get_version() -> *const c_char {
    const VERSION: &CStr =
        match CStr::from_bytes_with_nul(concat!(env!("CARGO_PKG_VERSION"), "\0").as_bytes()) {
            Ok(version) => version,
            // Fallback to "unknown" if version string is malformed
            Err(_) => c"unknown",
        };

    VERSION.as_ptr()
}

/// Calculates the CRC-32/ISCSI checksum (commonly called "crc32c" in many, but not all,
/// implementations).
///
/// https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iscsi
///
/// Returns 0 on error (e.g. null data pointer)
#[no_mangle]
pub extern "C" fn crc_fast_crc32_iscsi(data: *const c_char, len: usize) -> u32 {
    if data.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }
    clear_last_error();
    unsafe {
        #[allow(clippy::unnecessary_cast)]
        let bytes = slice::from_raw_parts(data as *const u8, len);
        crate::crc32_iscsi(bytes)
    }
}

/// Calculates the CRC-32/ISO-HDLC checksum (commonly called "crc32" in many, but not all,
/// implementations).
///
/// https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iso-hdlc
///
/// Returns 0 on error (e.g. null data pointer)
#[no_mangle]
pub extern "C" fn crc_fast_crc32_iso_hdlc(data: *const c_char, len: usize) -> u32 {
    if data.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }
    clear_last_error();
    unsafe {
        #[allow(clippy::unnecessary_cast)]
        let bytes = slice::from_raw_parts(data as *const u8, len);
        crate::crc32_iso_hdlc(bytes)
    }
}

/// Calculates the CRC-64/NVME checksum.
///
/// https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-nvme
///
/// Returns 0 on error (e.g. null data pointer)
#[no_mangle]
pub extern "C" fn crc_fast_crc64_nvme(data: *const c_char, len: usize) -> u64 {
    if data.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }
    clear_last_error();
    unsafe {
        #[allow(clippy::unnecessary_cast)]
        let bytes = slice::from_raw_parts(data as *const u8, len);
        crate::crc64_nvme(bytes)
    }
}

unsafe fn convert_to_string(data: *const u8, len: usize) -> String {
    if data.is_null() {
        return String::new();
    }

    // Safely construct string slice from raw parts
    match core::str::from_utf8(slice::from_raw_parts(data, len)) {
        Ok(s) => s.to_string(),
        Err(_) => String::new(), // Return empty string for invalid UTF-8
    }
}

fn get_or_leak_string(s: &str) -> &'static str {
    let cache = STRING_CACHE.call_once(|| Mutex::new(HashSet::new()));
    let mut cache = cache.lock();

    // Check if we already have this string
    if let Some(&cached) = cache.get(s) {
        return cached;
    }

    // Leak it and cache the result
    let leaked: &'static str = Box::leak(s.to_string().into_boxed_str());
    cache.insert(leaked);
    leaked
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    fn error_str(error: CrcFastError) -> String {
        unsafe {
            let ptr = crc_fast_error_message(error);
            assert!(!ptr.is_null(), "message for {error:?} must be non-null");
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }

    #[test]
    fn error_messages_and_last_error_cycle() {
        crc_fast_clear_error();
        assert_eq!(crc_fast_get_last_error(), CrcFastError::Success);
        assert!(error_str(CrcFastError::Success).contains("successfully"));
        assert!(error_str(CrcFastError::NullPointer).contains("Null pointer"));
        assert!(error_str(CrcFastError::InvalidKeyCount).contains("key count"));
        assert!(error_str(CrcFastError::UnsupportedWidth).contains("width"));
        assert!(error_str(CrcFastError::InvalidUtf8).contains("UTF-8"));
        assert!(error_str(CrcFastError::IoError).contains("I/O"));
        assert!(error_str(CrcFastError::StringConversionError).contains("string"));

        // Null data pointer records NullPointer.
        assert_eq!(
            crc_fast_checksum(CrcFastAlgorithm::Crc32IsoHdlc, core::ptr::null(), 9),
            0
        );
        assert_eq!(crc_fast_get_last_error(), CrcFastError::NullPointer);
        crc_fast_clear_error();
        assert_eq!(crc_fast_get_last_error(), CrcFastError::Success);
    }

    #[test]
    fn checksum_matches_rust_and_guards_null() {
        let data = b"123456789";
        let expected = crate::checksum(CrcAlgorithm::Crc32IsoHdlc, data);
        let actual = crc_fast_checksum(
            CrcFastAlgorithm::Crc32IsoHdlc,
            data.as_ptr() as *const c_char,
            data.len(),
        );
        assert_eq!(actual, expected);
        assert_eq!(crc_fast_get_last_error(), CrcFastError::Success);

        assert_eq!(crc_fast_crc32_iscsi(core::ptr::null(), data.len()), 0);
        assert_eq!(crc_fast_get_last_error(), CrcFastError::NullPointer);
        crc_fast_clear_error();
        assert_eq!(
            crc_fast_crc32_iso_hdlc(data.as_ptr() as *const c_char, data.len()),
            crate::crc32_iso_hdlc(data)
        );
        assert_eq!(
            crc_fast_crc64_nvme(data.as_ptr() as *const c_char, data.len()),
            crate::crc64_nvme(data)
        );
        assert_eq!(crc_fast_crc64_nvme(core::ptr::null(), data.len()), 0);
        crc_fast_clear_error();
    }

    #[test]
    fn digest_lifecycle_matches_rust() {
        let data = b"123456789";
        let handle = crc_fast_digest_new(CrcFastAlgorithm::Crc32IsoHdlc);
        assert!(!handle.is_null());
        crc_fast_digest_update(handle, data.as_ptr() as *const c_char, data.len());
        assert_eq!(crc_fast_get_last_error(), CrcFastError::Success);
        assert_eq!(crc_fast_digest_get_amount(handle), 9);
        assert_eq!(
            crc_fast_digest_finalize(handle),
            crate::checksum(CrcAlgorithm::Crc32IsoHdlc, data)
        );

        // finalize_reset returns the same value then clears the count.
        crc_fast_digest_reset(handle);
        crc_fast_digest_update(handle, data.as_ptr() as *const c_char, data.len());
        let once = crc_fast_digest_finalize_reset(handle);
        assert_eq!(once, 0xcbf43926);
        assert_eq!(crc_fast_digest_get_amount(handle), 0);

        // Combine two halves equals the whole input.
        let first = crc_fast_digest_new(CrcFastAlgorithm::Crc32IsoHdlc);
        let second = crc_fast_digest_new(CrcFastAlgorithm::Crc32IsoHdlc);
        crc_fast_digest_update(first, b"1234".as_ptr() as *const c_char, 4);
        crc_fast_digest_update(second, b"56789".as_ptr() as *const c_char, 5);
        crc_fast_digest_combine(first, second);
        assert_eq!(crc_fast_digest_finalize(first), 0xcbf43926);
        assert_eq!(crc_fast_digest_get_state(first), 0x340bc6d9);

        crc_fast_digest_free(first);
        crc_fast_digest_free(second);
        crc_fast_digest_free(handle);
    }

    #[test]
    fn digest_null_handles_set_error_without_crashing() {
        crc_fast_digest_update(core::ptr::null_mut(), b"x".as_ptr() as *const c_char, 1);
        assert_eq!(crc_fast_get_last_error(), CrcFastError::NullPointer);
        crc_fast_digest_update(
            crc_fast_digest_new(CrcFastAlgorithm::Crc32IsoHdlc),
            core::ptr::null(),
            1,
        );
        // Leaked handle from the line above is intentional for this guard test;
        // the important part is no crash and error recorded.
        assert_eq!(crc_fast_get_last_error(), CrcFastError::NullPointer);
        assert_eq!(crc_fast_digest_finalize(core::ptr::null_mut()), 0);
        assert_eq!(crc_fast_digest_finalize_reset(core::ptr::null_mut()), 0);
        assert_eq!(crc_fast_digest_get_amount(core::ptr::null_mut()), 0);
        assert_eq!(crc_fast_digest_get_state(core::ptr::null_mut()), 0);
        crc_fast_digest_free(core::ptr::null_mut());
        crc_fast_digest_reset(core::ptr::null_mut());
        crc_fast_digest_combine(core::ptr::null_mut(), core::ptr::null_mut());
        crc_fast_clear_error();
    }

    #[test]
    fn custom_params_version_and_target() {
        let params = crc_fast_get_custom_params(
            core::ptr::null(),
            32,
            0x04c11db7,
            0xffffffff,
            true,
            0xffffffff,
            0xcbf43926,
        );
        assert_eq!(crc_fast_get_last_error(), CrcFastError::Success);
        assert!(!params.keys.is_null());
        assert_eq!(params.key_count, 23);

        let data = b"123456789";
        let via_ffi = crc_fast_checksum_with_params(
            crc_fast_get_custom_params(
                core::ptr::null(),
                32,
                0x04c11db7,
                0xffffffff,
                true,
                0xffffffff,
                0xcbf43926,
            ),
            data.as_ptr() as *const c_char,
            data.len(),
        );
        assert_eq!(via_ffi, 0xcbf43926);

        // Unsupported width still returns params but records the error.
        let _bad = crc_fast_get_custom_params(core::ptr::null(), 7, 0x07, 0x00, false, 0x00, 0x00);
        assert_eq!(crc_fast_get_last_error(), CrcFastError::UnsupportedWidth);
        crc_fast_clear_error();

        // Invalid key count is rejected.
        let mut invalid = params;
        invalid.key_count = 3;
        assert_eq!(
            crc_fast_checksum_with_params(invalid, data.as_ptr() as *const c_char, data.len()),
            0
        );
        assert_eq!(crc_fast_get_last_error(), CrcFastError::InvalidKeyCount);
        crc_fast_clear_error();

        // Null name falls back to "custom"; invalid UTF-8 records an error.
        let name = CString::new("ffi-custom").unwrap();
        let named = crc_fast_get_custom_params(
            name.as_ptr(),
            32,
            0x04c11db7,
            0xffffffff,
            true,
            0xffffffff,
            0xcbf43926,
        );
        assert!(!named.keys.is_null());
        // NUL-terminated but invalid UTF-8: CStr::from_ptr needs the terminator.
        let bad_bytes = [0xffu8, 0x00];
        let _bad_name = crc_fast_get_custom_params(
            bad_bytes.as_ptr() as *const c_char,
            32,
            0x04c11db7,
            0xffffffff,
            true,
            0xffffffff,
            0xcbf43926,
        );
        assert_eq!(crc_fast_get_last_error(), CrcFastError::InvalidUtf8);
        crc_fast_clear_error();

        unsafe {
            let version = CStr::from_ptr(crc_fast_get_version());
            assert_eq!(version.to_bytes(), env!("CARGO_PKG_VERSION").as_bytes());

            let target = crc_fast_get_calculator_target(CrcFastAlgorithm::Crc32IsoHdlc);
            assert!(!target.is_null());
            assert!(!CStr::from_ptr(target).to_bytes().is_empty());
            let _ = CString::from_raw(target as *mut c_char);
        }
    }

    #[test]
    fn checksum_combine_matches_rust() {
        let first = crate::checksum(CrcAlgorithm::Crc32IsoHdlc, b"1234");
        let second = crate::checksum(CrcAlgorithm::Crc32IsoHdlc, b"56789");
        assert_eq!(
            crc_fast_checksum_combine(CrcFastAlgorithm::Crc32IsoHdlc, first, second, 5),
            0xcbf43926
        );

        let params = crc_fast_get_custom_params(
            core::ptr::null(),
            32,
            0x04c11db7,
            0xffffffff,
            true,
            0xffffffff,
            0xcbf43926,
        );
        assert!(!params.keys.is_null());
        assert_eq!(params.key_count, 23);
        crc_fast_clear_error();
        assert_eq!(
            crc_fast_checksum_combine_with_params(
                crc_fast_get_custom_params(
                    core::ptr::null(),
                    32,
                    0x04c11db7,
                    0xffffffff,
                    true,
                    0xffffffff,
                    0xcbf43926,
                ),
                first,
                second,
                5
            ),
            0xcbf43926
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn checksum_file_error_paths() {
        let missing = CString::new("/definitely/not/here/crc-fast-missing.txt").unwrap();
        let miss_bytes = missing.to_bytes();
        assert_eq!(
            crc_fast_checksum_file(
                CrcFastAlgorithm::Crc32IsoHdlc,
                miss_bytes.as_ptr(),
                miss_bytes.len()
            ),
            0
        );
        assert_eq!(crc_fast_get_last_error(), CrcFastError::IoError);
        crc_fast_clear_error();
        assert_eq!(
            crc_fast_checksum_file_with_params(
                crc_fast_get_custom_params(
                    core::ptr::null(),
                    32,
                    0x04c11db7,
                    0xffffffff,
                    true,
                    0xffffffff,
                    0xcbf43926,
                ),
                miss_bytes.as_ptr(),
                miss_bytes.len()
            ),
            0
        );
        assert_eq!(crc_fast_get_last_error(), CrcFastError::IoError);
        crc_fast_clear_error();
        assert_eq!(
            crc_fast_checksum_file(CrcFastAlgorithm::Crc32IsoHdlc, core::ptr::null(), 0),
            0
        );
        assert_eq!(crc_fast_get_last_error(), CrcFastError::NullPointer);
        crc_fast_clear_error();
    }
}
