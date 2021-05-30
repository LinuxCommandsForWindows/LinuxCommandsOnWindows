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
        minwindef::{
            DWORD,
            HKEY,
        },
        ntdef::NULL,
        rpcdce::{
            RPC_C_AUTHN_LEVEL_CALL,
            RPC_C_AUTHN_WINNT,
            RPC_C_AUTHZ_NONE,
            RPC_C_IMP_LEVEL_IMPERSONATE
        },
        winerror::{
            ERROR_SUCCESS,
            S_OK
        },
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
        },
        winnt::{
            KEY_READ,
            PVOID
        },
        winreg::{
            HKEY_LOCAL_MACHINE,
            RegCloseKey as Win32_RegCloseKey,
            RegGetValueW as Win32_RegGetValueW,
            RegOpenKeyExW as Win32_RegOpenKeyExW,
            RRF_RT_REG_SZ
        },
    }
};

use crate::error::{
    WinfetchError,
    WinfetchResult
};

pub struct OS {
    DisplayVersion: OsString,
    OSArchitecture: OsString,
    ProductName: OsString
}

impl OS {
    pub fn GetOperatingSystemVersion() -> WinfetchResult<Self> {
        let mut current_version_hkey: HKEY = ptr::null_mut();
        let current_version_key = AsRef::<OsStr>::as_ref(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();

        let current_version_handle = unsafe {
            match Win32_RegOpenKeyExW(
                HKEY_LOCAL_MACHINE,
                current_version_key.as_ptr(),
                0,
                KEY_READ,
                &mut current_version_hkey
            ) as DWORD {
                ERROR_SUCCESS => current_version_hkey,
                error_code => {
                    Win32_RegCloseKey(HKEY_LOCAL_MACHINE);  // close the key as we are aborting from the retrieving process
                    return Err(WinfetchError(
                        format!("could not get registry key from `HKEY_LOCAL_MACHINE`. (exit code: {})", error_code)
                    ));
                }
            }
        };

        let product_name_key = AsRef::<OsStr>::as_ref("ProductName")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();

        let mut buffer_len_product_name: DWORD = 0;

        unsafe {
            match Win32_RegGetValueW(
                current_version_handle,
                ptr::null_mut(),
                product_name_key.as_ptr(),
                RRF_RT_REG_SZ,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut buffer_len_product_name
            ) as DWORD {
                ERROR_SUCCESS => (),
                error_code => {
                    Win32_RegCloseKey(current_version_handle);  // close the key as we are aborting from the retrieving process
                    Win32_RegCloseKey(HKEY_LOCAL_MACHINE);  // close the key as we are aborting from the retrieving process
                    return Err(WinfetchError(
                        format!("could not get buffer length from `HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProductName`; error code: {}", error_code)
                    ));
                }
            }
        }

        let mut buffer_product_name = vec![0u16; buffer_len_product_name as usize];

        let product_name = unsafe {
            match Win32_RegGetValueW(
                current_version_handle,
                ptr::null_mut(),
                product_name_key.as_ptr(),
                RRF_RT_REG_SZ,
                ptr::null_mut(),
                buffer_product_name.as_mut_ptr() as PVOID,
                &mut buffer_len_product_name
            ) as DWORD {
                ERROR_SUCCESS => buffer_product_name,
                error_code => {
                    Win32_RegCloseKey(current_version_handle);  // close the key as we are aborting from the retrieving process
                    Win32_RegCloseKey(HKEY_LOCAL_MACHINE);  // close the key as we are aborting from the retrieving process
                    return Err(WinfetchError(
                        format!("could not get registry key from `HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProductName`; error code: {}", error_code)
                    ));
                }
            }
        };

        let display_version_key = AsRef::<OsStr>::as_ref("DisplayVersion")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();

        let mut buffer_len_display_version: DWORD = 0;

        unsafe {
            match Win32_RegGetValueW(
                current_version_handle,
                ptr::null_mut(),
                display_version_key.as_ptr(),
                RRF_RT_REG_SZ,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut buffer_len_display_version
            ) as DWORD {
                ERROR_SUCCESS => (),
                error_code => {
                    Win32_RegCloseKey(current_version_handle);  // close the key as we are aborting from the retrieving process
                    Win32_RegCloseKey(HKEY_LOCAL_MACHINE);  // close the key as we are aborting from the retrieving process
                    return Err(WinfetchError(
                        format!("could not get buffer length from `HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\DisplayVersion`; error code: {}", error_code)
                    ));
                }
            }
        }

        let mut buffer_display_version = vec![0u16; buffer_len_display_version as usize];

        let display_version = unsafe {
            match Win32_RegGetValueW(
                current_version_handle,
                ptr::null_mut(),
                display_version_key.as_ptr(),
                RRF_RT_REG_SZ,
                ptr::null_mut(),
                buffer_display_version.as_mut_ptr() as PVOID,
                &mut buffer_len_display_version
            ) as DWORD {
                ERROR_SUCCESS => buffer_display_version,
                error_code => {
                    Win32_RegCloseKey(current_version_handle);  // close the key as we are aborting from the retrieving process
                    Win32_RegCloseKey(HKEY_LOCAL_MACHINE);  // close the key as we are aborting from the retrieving process
                    return Err(WinfetchError(
                        format!("could not get registry value from `HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\DisplayVersion`; error code: {}", error_code)
                    ));
                }
            }
        };

        unsafe {
            Win32_RegCloseKey(current_version_handle);  // close the key as we are done with it
            Win32_RegCloseKey(HKEY_LOCAL_MACHINE);  // close the key as we are done with it
        }

        Ok(Self {
            DisplayVersion: OsString::from(OsString::from_wide(display_version.as_slice()).into_string().unwrap().replace('\0', "")),
            OSArchitecture: OsString::new(),
            ProductName: OsString::from(OsString::from_wide(product_name.as_slice()).into_string().unwrap().replace('\0', ""))
        })
    }

    pub fn GetOperatingSystemArchitecture(&mut self) -> WinfetchResult<()> {
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
                    return Err(WinfetchError(format!("failed to connect to wbem server; error code: {}", error_code)))
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
        let mut operating_system_query = AsRef::<OsStr>::as_ref("SELECT * FROM Win32_OperatingSystem")
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

        let os_architecture_property_name = AsRef::<OsStr>::as_ref(r"OSArchitecture")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();
        let mut os_architecture_property = unsafe {
            mem::zeroed::<VARIANT>()
        };

        unsafe {
            match (*wbem_class_object_null).Get(
                os_architecture_property_name.as_ptr(),
                0,
                &mut os_architecture_property,
                ptr::null_mut(),
                ptr::null_mut()
            ) {
                S_OK => (),
                error_code => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("could not get `OSArchitecture` property of wbem class object; error code: {}", error_code)))
                }
            }
        }

        let variant_type = unsafe {
            os_architecture_property.n1.n2().vt
        };
        let variant_value = unsafe {
            match variant_type as u32 {
                VT_BSTR => {
                    let bstr_ptr = os_architecture_property.n1.n2().n3.bstrVal();
                    let mut i = 0;

                    while *bstr_ptr.offset(i) != 0 {
                        i += 1;
                    }

                    let slice = slice::from_raw_parts(*bstr_ptr, i as usize + 1);
                    OsString::from(OsString::from_wide(slice).into_string().unwrap().replace('\0', ""))
                },
                _ => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("code branch should be unreachable - the variant type is expected to be `VT_BSTR`")))
                }
            }
        };

        unsafe {
            Win32_VariantClear(&mut os_architecture_property);

            Win32_CoUninitialize();
        };

        self.OSArchitecture.push(variant_value);

        Ok(())
    }
}

impl fmt::Display for OS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, Version {} [{}]", self.ProductName.to_str().unwrap(), self.DisplayVersion.to_str().unwrap(), self.OSArchitecture.to_str().unwrap())
    }
}
