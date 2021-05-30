use std::{
    ffi::OsStr,
    fmt,
    mem,
    os::windows::ffi::{
        OsStrExt
    },
    ptr
};

use winapi::{
    shared::{
        minwindef::{
            DWORD,
            HKEY
        },
        winerror::ERROR_SUCCESS
    },
    um::{
        winnt::{
            KEY_READ,
            PVOID
        },
        winreg::{
            HKEY_LOCAL_MACHINE,
            RegCloseKey as Win32_RegCloseKey,
            RegGetValueW as Win32_RegGetValueW,
            RegOpenKeyExW as Win32_RegOpenKeyExW,
            RRF_RT_REG_DWORD
        }
    }
};

use crate::error::{
    WinfetchError,
    WinfetchResult
};

pub struct WindowsNTKernel {
    CurrentMajorVersionNumber: u32,
    CurrentMinorVersionNumber: u32,
    UBR: u32
}

impl WindowsNTKernel {
    pub fn GetCurrentWindowsNTKernelVersion() -> WinfetchResult<Self> {
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
                    Win32_RegCloseKey(HKEY_LOCAL_MACHINE);  // close the key as we are aborting the retrieving process
                    return Err(WinfetchError(
                        format!("could not get registry key from `HKEY_LOCAL_MACHINE`. (exit code: {})", error_code)
                    ));
                }
            }
        };

        let current_major_version_number_key = AsRef::<OsStr>::as_ref(r"CurrentMajorVersionNumber")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();
        let mut current_major_version_number: DWORD = 0;

        unsafe {
            match Win32_RegGetValueW(
                current_version_handle,
                ptr::null_mut(),
                current_major_version_number_key.as_ptr(),
                RRF_RT_REG_DWORD,
                ptr::null_mut(),
                &mut current_major_version_number as *mut u32 as PVOID,
                &mut (mem::size_of::<DWORD>() as u32)
            ) as DWORD {
                ERROR_SUCCESS => (),
                error_code => {
                    Win32_RegCloseKey(current_version_handle);  // close the key as we are aborting from the retrieving process
                    Win32_RegCloseKey(HKEY_LOCAL_MACHINE);  // close the key as we are aborting from the retrieving process
                    return Err(WinfetchError(
                        format!("could not get value from `HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\CurrentMajorVersionNumber`; error code: {}", error_code)
                    ));
                }
            }
        }

        let current_minor_version_number_key = AsRef::<OsStr>::as_ref(r"CurrentMinorVersionNumber")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();
        let mut current_minor_version_number: DWORD = 0;

        unsafe {
            match Win32_RegGetValueW(
                current_version_handle,
                ptr::null_mut(),
                current_minor_version_number_key.as_ptr(),
                RRF_RT_REG_DWORD,
                ptr::null_mut(),
                &mut current_minor_version_number as *mut u32 as PVOID,
                &mut (mem::size_of::<DWORD>() as u32)
            ) as DWORD {
                ERROR_SUCCESS => (),
                error_code => {
                    Win32_RegCloseKey(current_version_handle);  // close the key as we are aborting from the retrieving process
                    Win32_RegCloseKey(HKEY_LOCAL_MACHINE);  // close the key as we are aborting from the retrieving process
                    return Err(WinfetchError(
                        format!("could not get value from `HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\CurrentMinorVersionNumber`; error code: {}", error_code)
                    ));
                }
            }
        }

        let ubr_key = AsRef::<OsStr>::as_ref(r"UBR")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>();
        let mut ubr: DWORD = 0;

        unsafe {
            match Win32_RegGetValueW(
                current_version_handle,
                ptr::null_mut(),
                ubr_key.as_ptr(),
                RRF_RT_REG_DWORD,
                ptr::null_mut(),
                &mut ubr as *mut u32 as PVOID,
                &mut (mem::size_of::<DWORD>() as u32)
            ) as DWORD {
                ERROR_SUCCESS => (),
                error_code => {
                    Win32_RegCloseKey(current_version_handle);  // close the key as we are aborting from the retrieving process
                    Win32_RegCloseKey(HKEY_LOCAL_MACHINE);  // close the key as we are aborting from the retrieving process
                    return Err(WinfetchError(
                        format!("could not get value from `HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\UBR`; error code: {}", error_code)
                    ));
                }
            }
        }

        Ok(Self {
            CurrentMajorVersionNumber: current_major_version_number,
            CurrentMinorVersionNumber: current_minor_version_number,
            UBR: ubr
        })
    }
}

impl fmt::Display for WindowsNTKernel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.CurrentMajorVersionNumber, self.CurrentMinorVersionNumber, self.UBR)
    }
}