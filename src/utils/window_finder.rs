use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM},
    UI::WindowsAndMessaging::{EnumWindows, GetWindowLongW, GetWindowTextW, GWL_STYLE},
};

pub type EnumWindowPredicate = fn(&EnumWindowParam, HWND) -> bool;
pub struct EnumWindowParam {
    predicate: EnumWindowPredicate,
    window_handles: Vec<HWND>,
    continue_enum: bool,
}

impl EnumWindowParam {
    pub fn new(predicate: EnumWindowPredicate) -> Self {
        EnumWindowParam {
            predicate,
            window_handles: Vec::<HWND>::new(),
            continue_enum: true,
        }
    }
    pub fn clear(&mut self) {
        self.window_handles.clear();
    }

    pub fn filter(&mut self, hwnd: HWND) -> bool {
        let result = (self.predicate)(&self, hwnd);
        if result {
            self.window_handles.push(hwnd);
        }
        result
    }
}

pub struct WindowFinder;
impl WindowFinder {
    pub fn enum_window(enum_window_param: &mut EnumWindowParam) {
        unsafe extern "system" fn enum_window_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let enum_window_param: &mut EnumWindowParam = std::mem::transmute(lparam.0);
            // filter window handle
            enum_window_param.filter(hwnd);
            BOOL::from(enum_window_param.continue_enum)
        }

        // init enum paramater
        enum_window_param.continue_enum = true;
        enum_window_param.clear();

        unsafe {
            let pointer: isize = std::mem::transmute(enum_window_param);
            EnumWindows(Some(enum_window_proc), LPARAM(pointer));
        }
    }

    pub fn get_window_title(hwnd: HWND) -> String {
        let mut buffer: [u16; 256] = [0; 256];
        unsafe {
            GetWindowTextW(hwnd, &mut buffer);
        }
        String::from_utf16_lossy(&buffer)
    }

    pub fn get_window_style(hwnd: HWND) -> u32 {
        unsafe { GetWindowLongW(hwnd, GWL_STYLE) as u32 }
    }

}
