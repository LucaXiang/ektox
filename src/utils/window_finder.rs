use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM},
    UI::WindowsAndMessaging::EnumWindows,
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
        (self.predicate)(&self, hwnd)
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
}
