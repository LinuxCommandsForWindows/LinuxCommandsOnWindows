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

use chrono::prelude::{
    DateTime,
    Local,
    TimeZone
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

use crate::winfetch::error::{
    WinfetchError,
    WinfetchResult
};

pub struct SystemUptime {
    Days: u64,
    Hours: u64,
    Minutes: u64
}

impl SystemUptime {
    pub fn GetSystemUptime() -> WinfetchResult<Self> {
        let current_time: DateTime<Local> = Local::now();

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
                    return Err(WinfetchError(format!("failed to initialize com library; error code: {}", error_code)))
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

        let last_boot_up_time_property_name = AsRef::<OsStr>::as_ref(r"LastBootUpTime")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();
        let mut last_boot_up_time_property = unsafe {
            mem::zeroed::<VARIANT>()
        };

        unsafe {
            match (*wbem_class_object_null).Get(
                last_boot_up_time_property_name.as_ptr(),
                0,
                &mut last_boot_up_time_property,
                ptr::null_mut(),
                ptr::null_mut()
            ) {
                S_OK => (),
                error_code => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("could not get `LastBootUpTime` property of wbem class object; error code: {}", error_code)))
                }
            }
        }

        let variant_type = unsafe {
            last_boot_up_time_property.n1.n2().vt
        };
        let variant_value = unsafe {
            match variant_type as u32 {
                VT_BSTR => {
                    let bstr_ptr = last_boot_up_time_property.n1.n2().n3.bstrVal();
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

        let variant_string = variant_value.into_string().unwrap();
        let variant_string_chars = variant_string.chars().collect::<Vec<_>>();
        let year = variant_string_chars[0..=3].iter().collect::<String>();
        let month = variant_string_chars[4..=5].iter().collect::<String>();
        let day = variant_string_chars[6..=7].iter().collect::<String>();
        let hour = variant_string_chars[8..=9].iter().collect::<String>();
        let minute = variant_string_chars[10..=11].iter().collect::<String>();
        let second = variant_string_chars[12..=13].iter().collect::<String>();

        let last_boot_up_time: DateTime<Local> = Local.datetime_from_str(
            &*format!(
                "{} {} {} {}:{}:{}",
                day,
                match &*month {
                    "01" => "January",
                    "02" => "February",
                    "03" => "March",
                    "04" => "April",
                    "05" => "May",
                    "06" => "June",
                    "07" => "July",
                    "08" => "August",
                    "09" => "September",
                    "10" => "October",
                    "11" => "November",
                    "12" => "December",
                    _ => return Err(WinfetchError(format!("invalid month, expected value within 1 and 12 (inclusive); actual value: {}", month)))
                },
                year,
                hour,
                minute,
                second
            ),
            "%d %B %Y %H:%M:%S"
        )
            .unwrap();
        let difference = current_time - last_boot_up_time;

        unsafe {
            Win32_VariantClear(&mut last_boot_up_time_property);

            Win32_CoUninitialize();
        };

        let days = difference.num_days();
        let hours = difference.num_hours();

        Ok(Self {
            Days: days as u64,
            Hours: (hours - days * 24) as u64,
            Minutes: (difference.num_minutes() - hours * 60) as u64
        })
    }
}

impl fmt::Display for SystemUptime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} day(s) {} hour(s) {} minute(s)", self.Days, self.Hours, self.Minutes)
    }
}