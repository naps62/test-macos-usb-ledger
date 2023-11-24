// Allow use of deprecated items, we defined ourselfes...
#![allow(deprecated)]

#[cfg(feature = "linux-static-rusb")]
extern crate rusb;

extern crate libc;

mod error;
mod ffi;

use std::ffi::CString;
use std::fmt;
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub use error::HidError;

pub type HidResult<T> = Result<T, HidError>;

/// Hidapi context and device member, which ensures deinitialization
/// of the C library happens, when, and only when all devices and the api instance is dropped.
struct HidApiLock;

impl HidApiLock {
    fn acquire() -> HidResult<HidApiLock> {
        const EXPECTED_CURRENT: bool = false;

        if EXPECTED_CURRENT
            == HID_API_LOCK.compare_and_swap(EXPECTED_CURRENT, true, Ordering::SeqCst)
        {
            // Initialize the HID and prevent other HIDs from being created
            unsafe {
                // This option must be set for Android Termux
                #[cfg(target_os = "android")]
                rusb::ffi::libusb_set_option(
                    std::ptr::null_mut(),
                    rusb::ffi::constants::LIBUSB_OPTION_WEAK_AUTHORITY,
                );

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

/// Object for handling hidapi context and implementing RAII for it.
/// Only one instance can exist at a time.
pub struct HidApi {
    _lock: Arc<HidApiLock>,
}

static HID_API_LOCK: AtomicBool = AtomicBool::new(false);

impl HidApi {
    /// Initializes the hidapi.
    ///
    /// Will also initialize the currently available device list.
    pub fn new() -> HidResult<Self> {
        let lock = HidApiLock::acquire()?;

        Ok(HidApi {
            _lock: Arc::new(lock),
        })
    }
}

/// Storage for device related information
///
/// Deprecated. Use `HidApi::device_list()` instead.
#[derive(Debug, Clone)]
#[deprecated]
pub struct HidDeviceInfo {
    pub path: CString,
    pub vendor_id: u16,
    pub product_id: u16,
    pub release_number: u16,
    pub usage_page: u16,
    pub usage: u16,
    pub interface_number: i32,
}

/// Device information. Use accessors to extract information about Hid devices.
///
/// Note: Methods like `serial_number()` may return None, if the conversion to a
/// String failed internally. You can however access the raw hid representation of the
/// string by calling `serial_number_raw()`
#[derive(Clone)]
pub struct DeviceInfo {
    path: CString,
    vendor_id: u16,
    product_id: u16,
    release_number: u16,
    usage_page: u16,
    usage: u16,
    interface_number: i32,
}

impl fmt::Debug for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HidDeviceInfo")
            .field("vendor_id", &self.vendor_id)
            .field("product_id", &self.product_id)
            .finish()
    }
}

impl Into<HidDeviceInfo> for DeviceInfo {
    fn into(self) -> HidDeviceInfo {
        HidDeviceInfo {
            path: self.path,
            vendor_id: self.vendor_id,
            product_id: self.product_id,
            release_number: self.release_number,
            usage_page: self.usage_page,
            usage: self.usage,
            interface_number: self.interface_number,
        }
    }
}

/// Object for accessing HID device
pub struct HidDevice {
    _hid_device: *mut ffi::HidDevice,
    /// Prevents this from outliving the api instance that created it
    _lock: ManuallyDrop<Arc<HidApiLock>>,
}

unsafe impl Send for HidDevice {}

impl Drop for HidDevice {
    fn drop(&mut self) {
        unsafe {
            ffi::hid_close(self._hid_device);
            ManuallyDrop::drop(&mut self._lock);
        };
    }
}
