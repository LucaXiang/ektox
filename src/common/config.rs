use serde::{Deserialize, Serialize};

use crate::utils::Hotkey;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub startup: bool,
    pub actions: Vec<Action>,
}

impl Config {
    pub fn get_actions(&self) -> &Vec<Action> {
        &self.actions
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub hotkey: Hotkey,
    pub exec: String,
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn it_works() {
        let data = r#"
        {
            "startup": true,
            "actions": [
              {
                "hotkey": "ctrl + 1",
                "exec": "C:/ProgramFile/test.exe"
              },
              {
                "hotkey": "ctrl + 2",
                "exec": "C:/ProgramFile/test.exe"
              }
            ]
        }"#;
        let config: Config = serde_json::from_str(data).unwrap();
        println!("result = {:#?}", config);
    }
    #[test]

    fn error() {
        let data = r#"
        {
            "startup": true,
            "actions": [
              {
                "hotkey": "ctrl + 1 + delete",
                "exec": "C:/ProgramFile/test.exe"
              },
              {
                "hotkey": "ctrl + 2",
                "exec": "C:/ProgramFile/test.exe"
              }
            ]
        }"#;
        let parse_result = serde_json::from_str::<Config>(data);
        if let Ok(_) = parse_result {
            panic!("should return error");
        }
    }
}
