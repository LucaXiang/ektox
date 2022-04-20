use windows::Win32::UI::Input::KeyboardAndMouse::*;
#[derive(Eq, Clone, Copy)]
#[allow(unused)]
pub enum SpecialKey {
    BackSpace = VK_BACK.0 as isize,
    Tab = VK_TAB.0 as isize,
    Clear = VK_CLEAR.0 as isize,
    Enter = VK_RETURN.0 as isize,
    Pause = VK_PAUSE.0 as isize,
    Caplock = VK_CAPITAL.0 as isize,
    Escape = VK_ESCAPE.0 as isize,
    SpaceBar = VK_SPACE.0 as isize,
    PageUp = VK_PRIOR.0 as isize,
    PageDown = VK_NEXT.0 as isize,
    End = VK_END.0 as isize,
    Home = VK_HOME.0 as isize,
    LeftArrow = VK_LEFT.0 as isize,
    UpArrow = VK_UP.0 as isize,
    RightArrow = VK_RIGHT.0 as isize,
    DownArrow = VK_DOWN.0 as isize,
    Select = VK_SELECT.0 as isize,
    Print = VK_PRINT.0 as isize,
    PrintScreen = VK_SNAPSHOT.0 as isize,
    Insert = VK_INSERT.0 as isize,
    Delete = VK_DELETE.0 as isize,
    F1 = VK_F1.0 as isize,
    F2 = VK_F2.0 as isize,
    F3 = VK_F3.0 as isize,
    F4 = VK_F4.0 as isize,
    F5 = VK_F5.0 as isize,
    F6 = VK_F6.0 as isize,
    F7 = VK_F7.0 as isize,
    F8 = VK_F8.0 as isize,
    F9 = VK_F9.0 as isize,
    F10 = VK_F10.0 as isize,
    F11 = VK_F11.0 as isize,
    F12 = VK_F12.0 as isize,
    NumLock = VK_NUMLOCK.0 as isize,
    ScrollLock = VK_SCROLL.0 as isize,
}

impl SpecialKey {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl PartialEq for SpecialKey {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
