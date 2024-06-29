use libc;
use std::io;

use scrap::Capturer;
use scrap::Display;

#[repr(C)]
pub struct ScrapSize {
    pub width: libc::c_int,
    pub height: libc::c_int,
}

#[repr(C)]
pub enum ScrapCaptureResult {
    ScrapCaptureSuccessful = 0,
    ScrapCaptureShouldSkip = 1,
    ScrapCaptureUnknown = 2,
    ScrapCaptureShouldReset = 3,
}

#[no_mangle]
pub extern "C" fn scrap_get_display_num() -> i32 {
    let displays = Display::all().unwrap();
    displays.len() as i32
}

#[no_mangle]
pub extern "C" fn scrap_get_display_size(index: libc::size_t, size: *mut ScrapSize) {
    let displays = Display::all().unwrap();
    let display = displays.get(index as usize).unwrap();

    unsafe {
        let size = size as *mut ScrapSize;
        (*size).width = display.width() as libc::c_int;
        (*size).height = display.height() as libc::c_int;
    }
}

#[no_mangle]
pub extern "C" fn scrap_create_capturer(index: i32) -> *mut libc::c_void {
    let mut displays = Display::all().unwrap();
    let d = displays.remove(index as usize);
    let capturer = Capturer::new(d).unwrap();
    Box::into_raw(Box::new(capturer)) as *mut libc::c_void
}

#[no_mangle]
pub extern "C" fn scrap_free_capturer(capturer: *mut libc::c_void) {
    unsafe {
        let capturer = capturer as *mut Capturer;
        drop(Box::from_raw(capturer));
    }

    println!("capturer freed.")
}

#[no_mangle]
pub extern "C" fn scrap_get_frame(
    capturer: *mut libc::c_void,
    dst: *mut libc::c_void,
    size: libc::size_t,
) -> ScrapCaptureResult {
    unsafe {
        let capturer = &mut *(capturer as *mut Capturer);
        let frame = capturer.frame();

        match frame {
            Ok(frame) => {
                let len = frame.len();

                if size != len {
                    return ScrapCaptureResult::ScrapCaptureUnknown;
                }

                libc::memcpy(dst, frame.as_ptr() as *const libc::c_void, len);
                return ScrapCaptureResult::ScrapCaptureSuccessful;
            }
            Err(e) => {
                println!("Error: {}", e);
                make_capture_result(e)
            }
        }
    }
}

fn make_capture_result(e: io::Error) -> ScrapCaptureResult {
    match e.kind() {
        io::ErrorKind::ConnectionReset => ScrapCaptureResult::ScrapCaptureShouldReset,
        io::ErrorKind::ConnectionAborted => ScrapCaptureResult::ScrapCaptureShouldReset,
        io::ErrorKind::InvalidData => ScrapCaptureResult::ScrapCaptureShouldReset,
        io::ErrorKind::WouldBlock => ScrapCaptureResult::ScrapCaptureShouldSkip,
        _ => ScrapCaptureResult::ScrapCaptureUnknown,
    }
}
