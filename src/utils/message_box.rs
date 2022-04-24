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
        // convert to c style string
        let mut text = text.to_owned();
        text.push('\0');
        let mut caption = caption.to_owned();
        caption.push('\0');

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

#[cfg(test)]
mod tests {
    use super::MessageBox;

    #[test]
    #[ignore = "message box will block test"]
    fn it_works() {
        MessageBox::info("Test!");
    }
}
