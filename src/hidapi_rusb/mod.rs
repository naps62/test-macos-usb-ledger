// Allow use of deprecated items, we defined ourselfes...
#![allow(deprecated)]

#[cfg(feature = "linux-static-rusb")]
extern crate rusb;

extern crate libc;

mod error;
mod ffi;

use std::sync::atomic::{AtomicBool, Ordering};

pub use error::HidError;

pub type HidResult<T> = Result<T, HidError>;

/// Hidapi context and device member, which ensures deinitialization
/// of the C library happens, when, and only when all devices and the api instance is dropped.
pub struct HidApiLock;

impl HidApiLock {
    pub fn acquire() -> HidResult<HidApiLock> {
        const EXPECTED_CURRENT: bool = false;

        if EXPECTED_CURRENT
            == HID_API_LOCK.compare_and_swap(EXPECTED_CURRENT, true, Ordering::SeqCst)
        {
            // Initialize the HID and prevent other HIDs from being created
            unsafe {
                // This option must be set for Android Termux
                if ffi::hid_init() == -1 {
                    HID_API_LOCK.store(false, Ordering::SeqCst);
                    return Err(HidError::InitializationError);
                }
                Ok(HidApiLock)
            }
        } else {
            Err(HidError::InitializationError)
        }
    }
}

impl Drop for HidApiLock {
    fn drop(&mut self) {
        unsafe {
            ffi::hid_exit();
        }
        HID_API_LOCK.store(false, Ordering::SeqCst);
    }
}

static HID_API_LOCK: AtomicBool = AtomicBool::new(false);
