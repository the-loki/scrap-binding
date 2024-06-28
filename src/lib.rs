use libc;

use scrap::Capturer;
use scrap::Display;

#[repr(C)]
pub struct ScrapSize {
    pub width: libc::c_int,
    pub height: libc::c_int,
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
) -> libc::c_int {
    unsafe {
        let capturer = &mut *(capturer as *mut Capturer);
        let frame = capturer.frame();

        match frame {
            Ok(frame) => {
                let len = frame.len();

                if size != len {
                    return 0;
                }

                libc::memcpy(dst, frame.as_ptr() as *const libc::c_void, len);
                return 1;
            }
            Err(e) => {
                println!("Error: {}", e);
                return 0;
            }
        }
    }
}
