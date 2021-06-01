use crate::winfetch::error::{
    WinfetchError,
    WinfetchResult
};

pub const ANSI_ESCAPE_SEQUENCE: &str = "\x1B[";

pub fn GeneratePercentageBar(percentage: i32) -> WinfetchResult<String> {
    if !(0..=100).contains(&percentage) {
        return Err(WinfetchError(format!("invalid percentage value; expected a value between 1 and 100 (inclusive), got {}", percentage)));
    }

    let mut percent_bar = String::from("[ ");
    let squares = percentage / 10;

    for i in 1..=squares {
        if i <= 6 {
            percent_bar.push_str(&format!("{}32m■{}0m", ANSI_ESCAPE_SEQUENCE, ANSI_ESCAPE_SEQUENCE));
        }
        else if i <= 8 {
            percent_bar.push_str(&format!("{}93m■{}0m", ANSI_ESCAPE_SEQUENCE, ANSI_ESCAPE_SEQUENCE));
        }
        else {
            percent_bar.push_str(&format!("{}91m■{}0m", ANSI_ESCAPE_SEQUENCE, ANSI_ESCAPE_SEQUENCE))
        }
    }

    percent_bar.push_str(&"-".repeat(10 - squares as usize));
    percent_bar.push_str(" ]");

    Ok(percent_bar)
}

pub fn GetWindowsASCIIArt() -> String {
    format!(
        " {}34m                    ....,,:;+ccllll
 {}34m      ...,,+:;  cllllllllllllllllll
 {}34m,cclllllllllll  lllllllllllllllllll
 {}34mllllllllllllll  lllllllllllllllllll
 {}34mllllllllllllll  lllllllllllllllllll
 {}34mllllllllllllll  lllllllllllllllllll
 {}34mllllllllllllll  lllllllllllllllllll
 {}34mllllllllllllll  lllllllllllllllllll
 {}34m
 {}34mllllllllllllll  lllllllllllllllllll
 {}34mllllllllllllll  lllllllllllllllllll
 {}34mllllllllllllll  lllllllllllllllllll
 {}34mllllllllllllll  lllllllllllllllllll
 {}34mllllllllllllll  lllllllllllllllllll
 {}34m`'ccllllllllll  lllllllllllllllllll
 {}34m      `' \\\\*::  :ccllllllllllllllll
 {}34m                       ````''*::cll
 {}34m                                 ``",
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE,
        ANSI_ESCAPE_SEQUENCE
    )
}
