use std::{
    ffi::OsStr,
    fmt,
    mem,
    os::windows::ffi::OsStrExt,
    ptr
};

use winapi::{
    shared::{
        minwindef::TRUE,
        ntdef::NULL,
        rpcdce::{
            RPC_C_AUTHN_LEVEL_CALL,
            RPC_C_AUTHN_WINNT,
            RPC_C_AUTHZ_NONE,
            RPC_C_IMP_LEVEL_IMPERSONATE
        },
        winerror::S_OK,
        wtypes::VT_I4,
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
        tlhelp32::{
            CreateToolhelp32Snapshot as Win32_CreateToolHelp32Snapshot,
            Process32First as Win32_Process32First,
            Process32Next as Win32_Process32Next,
            PROCESSENTRY32,
            TH32CS_SNAPPROCESS
        },
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

use crate::winfetch::{
    error::{
        WinfetchError,
        WinfetchResult,
    },
    utils
};

pub struct ProcessorUsage {
    LoadPercentage: i32,
    Processes: u64
}

impl ProcessorUsage {
    pub fn GetProcessorLoadPercentage() -> WinfetchResult<Self> {
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
        let mut processor_query = AsRef::<OsStr>::as_ref(r"SELECT * FROM Win32_Processor")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();

        let mut enum_wbem_class_object_null = NULL as *mut IEnumWbemClassObject;

        unsafe {
            match (*wbem_service_nonnull.as_ptr()).ExecQuery(
                wql.as_mut_ptr(),
                processor_query.as_mut_ptr(),
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

        let load_percentage_property_name = AsRef::<OsStr>::as_ref(r"LoadPercentage")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();
        let mut load_percentage_property = unsafe {
            mem::zeroed::<VARIANT>()
        };

        unsafe {
            match (*wbem_class_object_null).Get(
                load_percentage_property_name.as_ptr(),
                0,
                &mut load_percentage_property,
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

        let load_percentage_variant_type = unsafe {
            load_percentage_property.n1.n2().vt
        };
        let load_percentage_variant_value = unsafe {
            match load_percentage_variant_type as u32 {
                VT_I4 => {
                    let int_ptr = load_percentage_property.n1.n2().n3.intVal();
                    *int_ptr
                },
                _ => {
                    Win32_CoUninitialize();
                    return Err(WinfetchError(format!("code branch should be unreachable - the variant type is expected to be `VT_I4`")))
                }
            }
        };

        unsafe {
            Win32_VariantClear(&mut load_percentage_property);

            Win32_CoUninitialize();
        }

        Ok(Self {
            LoadPercentage: load_percentage_variant_value,
            Processes: 0
        })
    }

    pub fn GetProcessesCount(&mut self) -> WinfetchResult<()> {
        let handle = unsafe {
            Win32_CreateToolHelp32Snapshot(
                TH32CS_SNAPPROCESS,
                0
            )
        };
        let mut count = 0;

        if unsafe { Win32_Process32First(handle, &mut PROCESSENTRY32 {
            dwSize: mem::size_of::<PROCESSENTRY32>() as u32,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0i8; 260]
        }) } == TRUE {
            count += 1;
        }

        while unsafe { Win32_Process32Next(handle, &mut PROCESSENTRY32 {
            dwSize: mem::size_of::<PROCESSENTRY32>() as u32,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0i8; 260]
        }) } == TRUE {
            count += 1;
        }

        self.Processes = count;

        Ok(())
    }
}

impl fmt::Display for ProcessorUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} processes", utils::GeneratePercentageBar(self.LoadPercentage).unwrap(), self.Processes)
    }
}