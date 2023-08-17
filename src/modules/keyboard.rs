// more winapi stuf!!!

// ⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠ 
// ⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠
// ⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠
// ⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠⚠ HERE BE DRAGONS ⚠
// ⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠
// ⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠⚠🐉⚠

use std::thread;
use std::time::Duration;

use winapi::um::winuser::SendInput;

use winapi::um::winuser::KEYBDINPUT;
use winapi::um::winuser::INPUT_u;
use winapi::um::winuser::INPUT;
use winapi::um::winuser::INPUT_KEYBOARD;

/// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
pub fn send_key(key_code: u16, pressed: bool) {
    let mut input_u: INPUT_u = unsafe { std::mem::zeroed() };

    let mut flags: u32 = 0x0002;

    if pressed {
        flags = 0;
    }

    unsafe {
        *input_u.ki_mut() = KEYBDINPUT {
            wVk: key_code,
            dwExtraInfo: 0,
            wScan: 0,
            time: 0,
            dwFlags: flags
        }
    }

    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: input_u
    };
    let ipsize = std::mem::size_of::<INPUT>() as i32;
    unsafe {
        SendInput(1, &mut input, ipsize);
    };
}

/// shortcut function to press and release a key
/// 
/// note that this blocks the thread (probably a bad idea)
pub fn press_and_release(key_code: u16, duration: Duration) {
    send_key(key_code, true);
    thread::sleep(duration);
    send_key(key_code, false);
}