use ektox::common::Version;

fn main() {
    let version = Version::from_cargo_package();
    println!("{}", version);
}

/*
    let hotkey = Hotkey::parse("alt + 1").unwrap();
    unsafe {
        RegisterHotKey(HWND::default(), 1, hotkey.get_modifiers(), hotkey.get_key());
        let mut msg = MSG::default();

        while GetMessageW(&mut msg, HWND(0), 0, 0).into() {
            if (msg.message == WM_HOTKEY) {
                println!("{}", "pressed!");
                let id = msg.wParam;
                let mut str = "hotkey pressed\0";
                let buff = PCSTR(str.as_ptr());
                MessageBoxA(HWND(0), buff, PCSTR::default(), MB_OK);
            }
            DispatchMessageW(&msg);
        }
    }
*/
