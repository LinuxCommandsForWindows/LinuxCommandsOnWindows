use std::{
    ffi::OsString,
    fmt,
    os::windows::ffi::OsStringExt,
    slice
};

use winapi::um::winbase::{
    GetComputerNameW as Win32_GetComputerNameW,
    GetUserNameW as Win32_GetUserNameW,
};

use crate::winfetch::utils;

pub struct Names {
    pub ComputerName: OsString,
    pub UserName: OsString
}

impl Names {
    pub fn GetNames() -> Self {
        let mut computer_name_buffer = vec![0u16; 32767];
        let computer_name = unsafe {
            Win32_GetComputerNameW(computer_name_buffer.as_mut_ptr(), &mut 32767);

            let slice = slice::from_raw_parts(computer_name_buffer.as_ptr(), computer_name_buffer.len());
            OsString::from(OsString::from_wide(slice).into_string().unwrap().replace('\0', ""))
        };

        let mut user_name_buffer = vec![0u16; 32767];
        let user_name = unsafe {
            Win32_GetUserNameW(user_name_buffer.as_mut_ptr(), &mut 32767);

            let slice = slice::from_raw_parts(user_name_buffer.as_ptr(), user_name_buffer.len());
            OsString::from(OsString::from_wide(slice).into_string().unwrap().replace('\0', ""))
        };

        Self {
            ComputerName: computer_name,
            UserName: user_name
        }
    }
}

impl fmt::Display for Names {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}34m{}{}0m@{}34m{}{}0m",
               utils::ANSI_ESCAPE_SEQUENCE,
               self.UserName.clone().into_string().unwrap().to_ascii_lowercase(),
               utils::ANSI_ESCAPE_SEQUENCE,
               utils::ANSI_ESCAPE_SEQUENCE,
               self.ComputerName.clone().into_string().unwrap().to_ascii_lowercase(),
               utils::ANSI_ESCAPE_SEQUENCE)
    }
}
