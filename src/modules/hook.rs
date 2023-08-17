use std::ffi::CString;

use winapi;

use winapi::um::winuser::FindWindowA;
use winapi::um::winuser::GetWindowRect;

use winapi::shared::windef::RECT;

// winapi stuff for clone hero

// exe: "Clone Hero.exe"
// window: "Clone Hero"

// HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!!
// HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!!
// HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!! HERE BE DRAGONS!!!
// 🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉

pub unsafe fn get_ch_coords() -> RECT {
    // get ch hwnd
    let window_name = CString::new("Clone Hero").unwrap();     

    let hwnd = FindWindowA(std::ptr::null_mut(), window_name.as_ptr());

    // call getrect
    let mut rect = RECT {
        top: 0,
        left: 0,
        right: 0,
        bottom: 0
    };

    GetWindowRect(hwnd, &mut rect);

    rect
}