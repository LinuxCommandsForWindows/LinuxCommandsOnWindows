use std::fmt;

use winapi::{
    shared::{
        minwindef::{
            BOOL,
            LPARAM
        },
        ntdef::NULL,
        windef::{
            HDC,
            HMONITOR,
            LPRECT,
        }
    },
    um::{
        wingdi::{
            GetDeviceCaps as Win32_GetDeviceCaps,
            HORZRES,
            VERTRES
        },
        winuser::{
            EnumDisplayMonitors as Win32_EnumDisplayMonitors,
            GetDC as Win32_GetDC,
            ReleaseDC as Win32_ReleaseDC
        }
    }
};

use crate::winfetch::error::{
    WinfetchError,
    WinfetchResult
};

pub struct ScreenResolution {
    Resolutions: Vec<(i32, i32)>
}

impl ScreenResolution {
    pub fn GetScreenResolution() -> WinfetchResult<Self> {
        let mut resolutions = Vec::new();

        unsafe {
            let hdc = Win32_GetDC(NULL as *mut _);

            if Win32_EnumDisplayMonitors(
                hdc,
                NULL as *mut _,
                Some(MonitorEnumProc),
                &mut resolutions as *mut Vec<(i32, i32)> as isize
            ) == 0 {
                return Err(WinfetchError(String::from("enum display monitors returned a zero exit code")));
            }

            Win32_ReleaseDC(NULL as *mut _, hdc);
        }

        Ok(Self {
            Resolutions: resolutions
        })
    }
}

impl fmt::Display for ScreenResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let resolutions = self.Resolutions.iter().map(|(horizontal, vertical)| format!("{}x{}", horizontal, vertical)).collect::<Vec<_>>();

        write!(f, "{}", resolutions.join(", "))
    }
}

#[no_mangle]
pub(in self) unsafe extern "system" fn MonitorEnumProc(_: HMONITOR, hdc: HDC, _: LPRECT, resolutions: LPARAM) -> BOOL {
    let horizontal_resolution = Win32_GetDeviceCaps(hdc, HORZRES);
    let vertical_resolution = Win32_GetDeviceCaps(hdc, VERTRES);

    (*(resolutions as *mut Vec<(i32, i32)>)).push((horizontal_resolution, vertical_resolution));

    1
}
