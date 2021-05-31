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
            WBEM_S_FALSE,
            WBEM_S_NO_ERROR
        }
    }
};

use crate::error::{
    WinfetchError,
    WinfetchResult
};

pub struct GraphicsCard {
    Names: Vec<OsString>
}

impl GraphicsCard {
    pub fn GetGraphicsCards() -> WinfetchResult<Self> {
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
        let mut video_controller_query = AsRef::<OsStr>::as_ref(r"SELECT * FROM Win32_VideoController")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();

        let mut enum_wbem_class_object_null = NULL as *mut IEnumWbemClassObject;

        unsafe {
            match (*wbem_service_nonnull.as_ptr()).ExecQuery(
                wql.as_mut_ptr(),
                video_controller_query.as_mut_ptr(),
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

            if hresult == WBEM_S_FALSE as i32 {
                break;
            }

            let name_property_name = AsRef::<OsStr>::as_ref(r"Name")
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<u16>>();
            let mut name_property = unsafe {
                mem::zeroed::<VARIANT>()
            };

            unsafe {
                match (*wbem_class_object_null).Get(
                    name_property_name.as_ptr(),
                    0,
                    &mut name_property,
                    ptr::null_mut(),
                    ptr::null_mut()
                ) {
                    S_OK => (),
                    error_code => {
                        Win32_CoUninitialize();
                        return Err(WinfetchError(format!("could not get `Name` property of wbem class object; error code: {}", error_code)))
                    }
                }
            }

            let name_variant_type = unsafe {
                name_property.n1.n2().vt
            };
            vector.push(unsafe {
                match name_variant_type as u32 {
                    VT_BSTR => {
                        let bstr_ptr = name_property.n1.n2().n3.bstrVal();
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
            });

            unsafe {
                Win32_VariantClear(&mut name_property);
            }
        }

        unsafe {
            Win32_CoUninitialize();
        }

        Ok(Self {
            Names: vector
        })
    }
}

impl fmt::Display for GraphicsCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.Names.iter().map(|os_string| os_string.to_str().unwrap()).collect::<Vec<_>>().join(", "))
    }
}
