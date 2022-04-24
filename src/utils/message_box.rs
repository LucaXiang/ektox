use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{MessageBoxA, MB_OK, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE},
    },
};

pub struct MessageBox;

#[allow(unused)]
impl MessageBox {
    fn msg(text: &str, caption: &str, style: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT {
        unsafe {
            MessageBoxA(
                HWND(0),
                PCSTR(text.as_ptr()),
                PCSTR(caption.as_ptr()),
                style,
            )
        }
    }

    pub fn info(msg: &str) {
        Self::msg(msg, "Info", MB_OK);
    }

    pub fn erro(msg: &str) {
        Self::msg(msg, "Error", MB_OK);
    }
}
