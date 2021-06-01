use std::{
    ffi::OsString,
    os::windows::ffi::OsStringExt,
    slice,
};

use winapi::um::winbase::GetUserNameW as Win32_GetUserNameW;

pub const WHOAMI_HELP_MESSAGE: &str = r"Usage: whoami [OPTION]...
Prints the username associated with the current active user on the machine.

Options:
    --help          prints this message and exit
    --version       displays version information and exit

Source code for this command: <https://github.com/LinuxCommandsForWindows/LinuxCommandsOnWindows/tree/main/src/whoami>";

pub const WHOAMI_VERSION_MESSAGE: &str = r"whoami (Linux Commands for Windows) 0.1.0
Copyright (c) 2021 Linux Commands for Windows Project Developers
License Apache v2.0: <https://www.apache.org/licenses/LICENSE-2.0.txt>
This is free software: you are free to modify and redistribute it.
There is NO WARRANTY, to the extent permitted by law.";

pub fn GetUsername() -> String {
    let mut user_name_buffer = vec![0u16; 32767];

    unsafe {
        Win32_GetUserNameW(user_name_buffer.as_mut_ptr(), &mut 32767);

        let slice = slice::from_raw_parts(user_name_buffer.as_ptr(), user_name_buffer.len());
        OsString::from_wide(slice).into_string().unwrap().replace('\0', "").to_ascii_lowercase()
    }
}
