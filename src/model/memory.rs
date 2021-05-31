use std::{
    ffi::{
        OsStr,
        OsString
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

use crate::{
    error::{
        WinfetchError,
        WinfetchResult,
    },
    utils
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

        unsafe {
            match (*wbem_service_nonnull.as_ptr()).ExecQuery(
                wql.as_mut_ptr(),
                operating_system_query.as_mut_ptr(),
                (WBEM_FLAG_FORWARD_ONLY | WBEM_FLAG_RETURN_IMMEDIATELY) as i32,
                ptr::null_mut(),
                &mut enum_wbem_class_object_null
            ) {
                S_OK => (),
                error_code => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("failed to execute query; error code: {}", error_code)))
                }
            }
        }

        let mut wbem_class_object_null = NULL as *mut IWbemClassObject;
        let mut return_value = 0;

        unsafe {
            match (*enum_wbem_class_object_null).Next(
                WBEM_INFINITE as i32,
                1,
                &mut wbem_class_object_null,
                &mut return_value
            ) {
                S_OK => (),
                error_code => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("could not get next element of enumeration; error code: {}", error_code)))
                }
            }
        }

        let free_physical_memory_property_name = AsRef::<OsStr>::as_ref(r"FreePhysicalMemory")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();
        let total_visible_memory_size_property_name = AsRef::<OsStr>::as_ref(r"TotalVisibleMemorySize")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();

        let mut free_physical_memory_property = unsafe {
            mem::zeroed::<VARIANT>()
        };
        let mut total_visible_memory_size_property = unsafe {
            mem::zeroed::<VARIANT>()
        };

        unsafe {
            match (*wbem_class_object_null).Get(
                free_physical_memory_property_name.as_ptr(),
                0,
                &mut free_physical_memory_property,
                ptr::null_mut(),
                ptr::null_mut()
            ) {
                S_OK => (),
                error_code => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("could not get `FreePhysicalMemory` property of wbem class object; error code: {}", error_code)))
                }
            }

            match (*wbem_class_object_null).Get(
                total_visible_memory_size_property_name.as_ptr(),
                0,
                &mut total_visible_memory_size_property,
                ptr::null_mut(),
                ptr::null_mut()
            ) {
                S_OK => (),
                error_code => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("could not get `TotalVisibleMemorySize` property of wbem class object; error code: {}", error_code)))
                }
            }
        }

        let free_physical_memory_variant_type = unsafe {
            free_physical_memory_property.n1.n2().vt
        };
        let free_physical_memory_variant_value = unsafe {
            match free_physical_memory_variant_type as u32 {
                VT_BSTR => {
                    let bstr_ptr = free_physical_memory_property.n1.n2().n3.bstrVal();
                    let mut i = 0;

                    while *bstr_ptr.offset(i) != 0 {
                        i += 1;
                    }

                    let slice = slice::from_raw_parts(*bstr_ptr, i as usize + 1);
                    OsString::from(OsString::from_wide(slice).into_string().unwrap().replace('\0', ""))
                }
                _ => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("code branch should be unreachable - the variant type is expected to be `VT_BSTR`")))
                }
            }
        };

        let total_visible_memory_size_variant_type = unsafe {
            total_visible_memory_size_property.n1.n2().vt
        };
        let total_visible_memory_size_variant_value = unsafe {
            match total_visible_memory_size_variant_type as u32 {
                VT_BSTR => {
                    let bstr_ptr = total_visible_memory_size_property.n1.n2().n3.bstrVal();
                    let mut i = 0;

                    while *bstr_ptr.offset(i) != 0 {
                        i += 1;
                    }

                    let slice = slice::from_raw_parts(*bstr_ptr, i as usize + 1);
                    OsString::from(OsString::from_wide(slice).into_string().unwrap().replace('\0', ""))
                }
                _ => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("code branch should be unreachable - the variant type is expected to be `VT_BSTR`")))
                }
            }
        };

        unsafe {
            Win32_VariantClear(&mut free_physical_memory_property);
            Win32_VariantClear(&mut total_visible_memory_size_property);

            Win32_CoUninitialize();
        }

        Ok(Self {
            FreePhysicalMemory: free_physical_memory_variant_value.into_string().unwrap().parse().unwrap(),
            TotalVisibleMemorySize: total_visible_memory_size_variant_value.into_string().unwrap().parse().unwrap()
        })
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let usage = (self.TotalVisibleMemorySize - self.FreePhysicalMemory)  / 1024.0 / 1024.0;
        let total = self.TotalVisibleMemorySize / 1024.0 / 1024.0;

        write!(f, "{} {:.2} GB / {:.2} GB", utils::GeneratePercentageBar(((usage / total) * 100.0) as i32).unwrap(), usage, total)
    }
}
