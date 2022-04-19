use windows::Win32::{
    Foundation::{CloseHandle, BOOL, HINSTANCE, HWND, LPARAM, MAX_PATH},
    System::{
        ProcessStatus::K32GetModuleFileNameExW,
        Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION},
    },
    UI::WindowsAndMessaging::{
        EnumWindows, GetWindowLongW, GetWindowTextLengthW, GetWindowTextW,
        GetWindowThreadProcessId, GWL_STYLE,
    },
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
        static mut BUFFER: [u16; 256] = [0; 256];
        unsafe {
            let len = GetWindowTextLengthW(hwnd) as usize;
            GetWindowTextW(hwnd, &mut BUFFER);
            String::from_utf16_lossy(&BUFFER[0..len])
        }
    }

    pub fn get_window_style(hwnd: HWND) -> u32 {
        unsafe { GetWindowLongW(hwnd, GWL_STYLE) as u32 }
    }

    pub fn get_process_id_from_hwnd(hwnd: HWND) -> u32 {
        let mut pid: u32 = 0;
        unsafe {
            GetWindowThreadProcessId(hwnd, &mut pid);
        }
        pid
    }

    pub fn get_process_name_from_pid(pid: u32) -> String {
        const MAX_FILENAME: usize = MAX_PATH as usize;
        let mut buffer: [u16; MAX_FILENAME] = [0; MAX_FILENAME];
        unsafe {
            match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, BOOL::from(false), pid) {
                Ok(handle) => {
                    K32GetModuleFileNameExW(handle, HINSTANCE(0), &mut buffer);
                    CloseHandle(handle);
                }
                Err(_) => {}
            }
        }
        String::from_utf16_lossy(&buffer)
    }
}
