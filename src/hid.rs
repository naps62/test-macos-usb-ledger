//! Native HID APDU transport for Ledger Nano hardware wallets
use lazy_static::lazy_static;

use std::ffi::CString;

use hidapi_rusb::HidDevice;
use std::cell::RefCell;
use std::sync::{Arc, Mutex, Weak};

const LEDGER_VID: u16 = 0x2c97;

#[cfg(not(target_os = "linux"))]
const LEDGER_USAGE_PAGE: u16 = 0xFFA0;
const LEDGER_CHANNEL: u16 = 0x0101;
const LEDGER_PACKET_SIZE: u8 = 64;

const LEDGER_TIMEOUT: i32 = 10_000_000;

struct HidApiWrapper {
    _api: RefCell<Weak<Mutex<hidapi_rusb::HidApi>>>,
}

/// The transport struct. Holds a `Mutex` on the underlying `HidAPI` instance.
///
/// Instantiate with [`new`][TransportNativeHID::new].
pub struct TransportNativeHID {
    api_mutex: Arc<Mutex<hidapi_rusb::HidApi>>,
}

unsafe impl Send for HidApiWrapper {}

lazy_static! {
    static ref HIDAPIWRAPPER: Arc<Mutex<HidApiWrapper>> =
        Arc::new(Mutex::new(HidApiWrapper::new()));
}

impl HidApiWrapper {
    fn new() -> Self {
        HidApiWrapper {
            _api: RefCell::new(Weak::new()),
        }
    }

    fn get(&self) -> anyhow::Result<Arc<Mutex<hidapi_rusb::HidApi>>> {
        let tmp = self._api.borrow().upgrade();
        if let Some(api_mutex) = tmp {
            return Ok(api_mutex);
        }

        let hidapi = hidapi_rusb::HidApi::new()?;
        let tmp = Arc::new(Mutex::new(hidapi));
        self._api.replace(Arc::downgrade(&tmp));
        Ok(tmp)
    }
}

impl TransportNativeHID {
    #[cfg(not(target_os = "linux"))]
    fn find_ledger_device_path(api: &hidapi_rusb::HidApi) -> anyhow::Result<CString> {
        for device in api.device_list() {
            if device.vendor_id() == LEDGER_VID && device.usage_page() == LEDGER_USAGE_PAGE {
                return Ok(device.path().into());
            }
        }
        Err(anyhow::anyhow!("NativeTransportError::DeviceNotFound"))
    }

    #[cfg(target_os = "linux")]
    fn find_ledger_device_path(api: &hidapi_rusb::HidApi) -> anyhow::Result<CString> {
        for device in api.device_list() {
            if device.vendor_id() == LEDGER_VID {
                return Ok(device.path().into());
            }
        }
        Err(anyhow::anyhow!("error"))
    }

    /// Get the device path string
    #[allow(dead_code)]
    pub fn device_path(&self) -> anyhow::Result<CString> {
        Self::find_ledger_device_path(&self.api_mutex.lock().unwrap())
    }

    /// Get a new TransportNativeHID by acquiring a lock on the global `hidapi_rusb::HidAPI`.
    /// Note that this may block forever if the resource is in use.
    pub fn new() -> anyhow::Result<Self> {
        let apiwrapper = HIDAPIWRAPPER.lock().expect("Could not lock api wrapper");
        let api_mutex = apiwrapper.get().expect("Error getting api_mutex");
        let api = api_mutex.lock().expect("Could not lock");

        #[cfg(not(target_os = "android"))]
        let device = {
            let device_path = TransportNativeHID::find_ledger_device_path(&api)?;
            api.open_path(&device_path)?
        };

        let ledger = TransportNativeHID {
            api_mutex: api_mutex.clone(),
        };

        Ok(ledger)
    }

    // TODO: why does this exist?
    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn close(self) {}
}

/*******************************************************************************
*   (c) 2018 ZondaX GmbH
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
********************************************************************************/
