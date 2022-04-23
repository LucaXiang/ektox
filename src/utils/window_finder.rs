use windows::Win32::{
    Foundation::{CloseHandle, BOOL, HINSTANCE, HWND, LPARAM, MAX_PATH},
    System::{
        ProcessStatus::K32GetModuleFileNameExW,
        Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION},
    },
    UI::WindowsAndMessaging::{
        EnumWindows, GetWindow, GetWindowInfo, GetWindowLongW, GetWindowTextLengthW,
        GetWindowTextW, GetWindowThreadProcessId, GWL_EXSTYLE, GWL_STYLE, GW_OWNER, WINDOWINFO,
        WS_EX_TOOLWINDOW, WS_POPUP, WS_VISIBLE,
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
        let result = (self.predicate)(self, hwnd);
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
            let len = GetWindowTextW(hwnd, &mut BUFFER) as usize;
            String::from_utf16_lossy(&BUFFER[0..len])
        }
    }

    pub fn get_window_style(hwnd: HWND) -> u32 {
        unsafe { GetWindowLongW(hwnd, GWL_STYLE) as u32 }
    }

    pub fn get_window_extend_style(hwnd: HWND) -> u32 {
        unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) as u32 }
    }

    pub fn get_window_info(hwnd: HWND) -> WINDOWINFO {
        let mut window_info = WINDOWINFO::default();
        window_info.cbSize = std::mem::size_of::<WINDOWINFO>() as u32;
        unsafe {
            GetWindowInfo(hwnd, &mut window_info);
        }
        window_info
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
        static mut BUFFER: [u16; MAX_FILENAME] = [0; MAX_FILENAME];
        unsafe {
            let mut len = 0;
            if let Ok(handle) =
                OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, BOOL::from(false), pid)
            {
                len = K32GetModuleFileNameExW(handle, HINSTANCE(0), &mut BUFFER) as usize;
                CloseHandle(handle);
            }
            String::from_utf16_lossy(&BUFFER[0..len])
        }
    }

    pub fn get_process_name_from_hwnd(hwnd: HWND) -> String {
        let pid = Self::get_process_id_from_hwnd(hwnd);
        Self::get_process_name_from_pid(pid)
    }

    pub fn get_frontend_window() -> Vec<HWND> {
        let mut param = EnumWindowParam::new(|_ewp, hwnd| {
            let mut result = false;
            #[allow(clippy::never_loop)]
            loop {
                unsafe {
                    // window must have owner
                    if GetWindow(hwnd, GW_OWNER) != HWND(0) {
                        break;
                    }

                    // window title is not empty
                    if GetWindowTextLengthW(hwnd) == 0 {
                        break;
                    }
                }

                let window_info = Self::get_window_info(hwnd);

                let window_style = window_info.dwStyle;

                // window is not pop-up window.
                if window_style & WS_POPUP.0 != 0 {
                    break;
                }
                // window should to be visible
                if window_style & WS_VISIBLE.0 == 0 {
                    break;
                }

                let window_extend_style = window_info.dwExStyle;

                // window is not floating window
                if window_extend_style & WS_EX_TOOLWINDOW.0 != 0 {
                    break;
                }

                result = true;
                break;
            }
            result
        });
        WindowFinder::enum_window(&mut param);
        param.window_handles
    }
}

#[cfg(test)]
mod tests {
    use super::WindowFinder;

    #[test]
    #[ignore]
    fn get_frontend_window() {
        let r = WindowFinder::get_frontend_window();
        for window in r.into_iter() {
            eprintln!("HANDLE: {:x}", window.0);
            eprintln!("->TITLE: {:#?}", WindowFinder::get_window_title(window));
            eprintln!(
                "--> FILENAME:{:#?}\n",
                WindowFinder::get_process_name_from_hwnd(window)
            );
        }
    }
}
