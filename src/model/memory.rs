use std::{
    ffi::{
        OsStr,
        OsString,
    },
    fmt,
    mem,
    os::windows::ffi::{
        OsStrExt,
        OsStringExt
    },
    ptr,
    slice
};

use winapi::{
    shared::{
        ntdef::NULL,
        rpcdce::{
            RPC_C_AUTHN_LEVEL_CALL,
            RPC_C_AUTHN_WINNT,
            RPC_C_AUTHZ_NONE,
            RPC_C_IMP_LEVEL_IMPERSONATE
        },
        winerror::S_OK,
        wtypes::VT_BSTR,
        wtypesbase::CLSCTX_INPROC_SERVER
    },
    um::{
        combaseapi::{
            CoCreateInstance as Win32_CoCreateInstance,
            CoSetProxyBlanket as Win32_CoSetProxyBlanket,
            CoUninitialize as Win32_CoUninitialize
        },
        oaidl::VARIANT,
        objbase::CoInitialize as Win32_CoInitialize,
        objidl::EOAC_NONE,
        oleauto::VariantClear as Win32_VariantClear,
        wbemcli::{
            CLSID_WbemLocator,
            IEnumWbemClassObject,
            IID_IWbemLocator,
            IWbemClassObject,
            IWbemLocator,
            IWbemServices,
            WBEM_FLAG_FORWARD_ONLY,
            WBEM_FLAG_RETURN_IMMEDIATELY,
            WBEM_INFINITE
        }
    }
};

use crate::error::{
    WinfetchError,
    WinfetchResult
};

pub struct Memory {
    FreePhysicalMemory: f64,
    TotalVisibleMemorySize: f64
}

impl Memory {
    pub fn GetMemoryStatistics() -> WinfetchResult<Self> {
        let mut wbem_locator_c_void = NULL;

        unsafe {
            match Win32_CoInitialize(
                NULL
            ) {
                S_OK => (),
                error_code => return Err(WinfetchError(format!("failed to initialize com library; error code: {}", error_code)))
            }

            match Win32_CoCreateInstance(
                &CLSID_WbemLocator,
                ptr::null_mut(),
                CLSCTX_INPROC_SERVER,
                &IID_IWbemLocator,
                &mut wbem_locator_c_void
            ) {
                S_OK => (),
                error_code => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("failed to create wbem locator; error code: {}", error_code)))
                }
            }
        }

        let wbem_locator = ptr::NonNull::new(wbem_locator_c_void as *mut IWbemLocator).unwrap();
        let mut root_cimv2 = AsRef::<OsStr>::as_ref(r"ROOT\CIMV2")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();

        let mut wbem_service_null = ptr::null_mut::<IWbemServices>();

        unsafe {
            match (*wbem_locator.as_ptr()).ConnectServer(
                root_cimv2.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut wbem_service_null
            ) {
                S_OK => (),
                error_code => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("failed to connect wbem server; error code: {}", error_code)))
                }
            }
        }

        let wbem_service_nonnull = ptr::NonNull::new(wbem_service_null).unwrap();

        unsafe {
            match Win32_CoSetProxyBlanket(
                wbem_service_nonnull.as_ptr() as _,
                RPC_C_AUTHN_WINNT,
                RPC_C_AUTHZ_NONE,
                ptr::null_mut(),
                RPC_C_AUTHN_LEVEL_CALL,
                RPC_C_IMP_LEVEL_IMPERSONATE,
                NULL,
                EOAC_NONE
            ) {
                S_OK => (),
                error_code => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("failed to set proxy; error code: {}", error_code)))
                }
            }
        }

        let mut wql = AsRef::<OsStr>::as_ref(r"WQL")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();
        let mut operating_system_query = AsRef::<OsStr>::as_ref(r"SELECT * FROM Win32_OperatingSystem")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();

        let mut enum_wbem_class_object_null = NULL as *mut IEnumWbemClassObject;

        Ok(Self {
            FreePhysicalMemory: 0.0,
            TotalVisibleMemorySize: 0.0
        })
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} MB / {} MB", self.TotalVisibleMemorySize - self.FreePhysicalMemory, self.TotalVisibleMemorySize)
    }
}
