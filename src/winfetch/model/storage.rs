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
        winerror::{
            HRESULT,
            S_OK,
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
            WBEM_INFINITE,
            WBEM_S_NO_ERROR,
            WBEM_S_FALSE
        }
    }
};

use crate::winfetch::{
    __internals,
    error::{
        WinfetchError,
        WinfetchResult,
    },
    utils
};

pub struct Storage {
    pub Drives: Vec<StorageDrive>
}

impl Storage {
    pub fn GetStorageStatistics() -> WinfetchResult<Self> {
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
        let mut logical_disk_query = AsRef::<OsStr>::as_ref(r"SELECT * FROM Win32_LogicalDisk")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();

        let mut enum_wbem_class_object_null = NULL as *mut IEnumWbemClassObject;

        unsafe {
            match (*wbem_service_nonnull.as_ptr()).ExecQuery(
                wql.as_mut_ptr(),
                logical_disk_query.as_mut_ptr(),
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

        let mut hresult: HRESULT = WBEM_S_NO_ERROR as i32;
        let mut vector = Vec::new();

        while hresult == WBEM_S_NO_ERROR as i32 {
            let mut wbem_class_object_null = NULL as *mut IWbemClassObject;
            let mut return_value = 0;

            hresult = unsafe {
                (*enum_wbem_class_object_null).Next(
                    WBEM_INFINITE as i32,
                    1,
                    &mut wbem_class_object_null,
                    &mut return_value
                )
            };

            if hresult == WBEM_S_FALSE as i32 || wbem_class_object_null.is_null() {
                break;
            }

            let device_id_property_name = AsRef::<OsStr>::as_ref(r"DeviceID")
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<u16>>();
            let mut device_id_property = unsafe {
                mem::zeroed::<VARIANT>()
            };

            unsafe {
                match (*wbem_class_object_null).Get(
                    device_id_property_name.as_ptr(),
                    0,
                    &mut device_id_property,
                    ptr::null_mut(),
                    ptr::null_mut()
                ) {
                    S_OK => (),
                    error_code => {
                        Win32_CoUninitialize();
                        return Err(WinfetchError(format!("could not get `DeviceID` property of wbem class object; error code: {}", error_code)))
                    }
                }
            }

            let device_id_variant_type = unsafe {
                device_id_property.n1.n2().vt
            };
            let device_id_variant_value = unsafe {
                match device_id_variant_type as u32 {
                    VT_BSTR => {
                        let bstr_ptr = device_id_property.n1.n2().n3.bstrVal();
                        let mut i = 0;

                        while *bstr_ptr.offset(i) != 0 {
                            i += 1;
                        }

                        let slice = slice::from_raw_parts(*bstr_ptr, i as usize + 1);
                        OsString::from(format!("/mnt/{}/", OsString::from_wide(slice).into_string().unwrap().replace('\0', "")).replace(":", "").to_ascii_lowercase())
                    },
                    _ => {
                        Win32_CoUninitialize();
                        return Err(WinfetchError(format!("code branch should be unreachable - the variant type is expected to be `VT_BSTR`")))
                    }
                }
            };

            let free_space_property_name = AsRef::<OsStr>::as_ref(r"FreeSpace")
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<u16>>();
            let mut free_space_property = unsafe {
                mem::zeroed::<VARIANT>()
            };

            unsafe {
                match (*wbem_class_object_null).Get(
                    free_space_property_name.as_ptr(),
                    0,
                    &mut free_space_property,
                    ptr::null_mut(),
                    ptr::null_mut()
                ) {
                    S_OK => (),
                    error_code => {
                        Win32_CoUninitialize();
                        return Err(WinfetchError(format!("could not get `FreeSpace` property of wbem class object; error code: {}", error_code)))
                    }
                }
            }

            let free_space_variant_type = unsafe {
                free_space_property.n1.n2().vt
            };
            let free_space_variant_value = unsafe {
                match free_space_variant_type as u32 {
                    VT_BSTR => {
                        let bstr_ptr = free_space_property.n1.n2().n3.bstrVal();
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

            let size_property_name = AsRef::<OsStr>::as_ref(r"Size")
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<u16>>();
            let mut size_property = unsafe {
                mem::zeroed::<VARIANT>()
            };

            unsafe {
                match (*wbem_class_object_null).Get(
                    size_property_name.as_ptr(),
                    0,
                    &mut size_property,
                    ptr::null_mut(),
                    ptr::null_mut()
                ) {
                    S_OK => (),
                    error_code => {
                        Win32_CoUninitialize();
                        return Err(WinfetchError(format!("could not get `Size` property of wbem class object; error code: {}", error_code)))
                    }
                }
            }

            let size_variant_type = unsafe {
                size_property.n1.n2().vt
            };
            let size_variant_value = unsafe {
                match size_variant_type as u32 {
                    VT_BSTR => {
                        let bstr_ptr = size_property.n1.n2().n3.bstrVal();
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

            vector.push(StorageDrive {
                DriveLetter: device_id_variant_value,
                FreeSpace: free_space_variant_value.into_string().unwrap().parse().unwrap(),
                Size: size_variant_value.into_string().unwrap().parse().unwrap()
            });

            unsafe {
                Win32_VariantClear(&mut device_id_property);
                Win32_VariantClear(&mut free_space_property);
                Win32_VariantClear(&mut size_property);
            }
        }

        Ok(Self {
            Drives: vector
        })
    }
}

impl fmt::Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for drive in &self.Drives {
            write!(f, "{}", drive)?;
        }

        Ok(())
    }
}

pub struct StorageDrive {
    DriveLetter: OsString,
    FreeSpace: f64,
    Size: f64
}

impl fmt::Display for StorageDrive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let used = self.Size - self.FreeSpace;
        let total = self.Size;

        write!(
            f,
            "{}34m Drive ({}){}0m: {} {} / {}",
            utils::ANSI_ESCAPE_SEQUENCE,
            self.DriveLetter.clone().into_string().unwrap(),
            utils::ANSI_ESCAPE_SEQUENCE,
            utils::GeneratePercentageBar(((used / total) * 100.0) as i32).unwrap(),
            __internals::__InternalsToUnits(used),
            __internals::__InternalsToUnits(total)
        )
    }
}
