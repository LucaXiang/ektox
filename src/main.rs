use std::{collections::HashMap, fs::File, io::BufReader};

use ektox::{
    common::{App, Config},
    utils::{Hotkey, WindowFinder},
};

use windows::{
    Win32::{
        Foundation::HWND,
        UI::{
            Input::KeyboardAndMouse::RegisterHotKey,
            WindowsAndMessaging::{
                DispatchMessageW, GetMessageW, SetForegroundWindow, MSG,
                WM_HOTKEY,
            },
        },
    },
};

fn main() {
    let file = File::open("./config.json").unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).unwrap();
    let hotkey = Hotkey::parse(config.actions[0].hotkey.as_str()).unwrap();
    let windows = WindowFinder::get_frontent_window();
    let mut maps = HashMap::<String, HWND>::new();
    for w in windows {
        let filename =
            WindowFinder::get_process_name_from_pid(WindowFinder::get_process_id_from_hwnd(w));

        let filename = filename
            .chars()
            .map(|c| -> char {
                if c == '\\' {
                    '/'
                } else {
                    c
                }
            })
            .collect();
        maps.insert(filename, w);
    }
    unsafe {
        RegisterHotKey(HWND::default(), 1, hotkey.get_modifiers(), hotkey.get_key());
        let mut msg = MSG::default();

        while GetMessageW(&mut msg, HWND(0), 0, 0).into() {
            if msg.message == WM_HOTKEY {
                let exec = &config.actions[0].exec;
                let w = maps[exec];
                println!(
                    "pressed {:} switch to => {}",
                    config.actions[0].hotkey, exec
                );
                SetForegroundWindow(w);
            }
            DispatchMessageW(&msg);
        }
    }
    let app = App::new();
    println!("{}", app.get_version());
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
