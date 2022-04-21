use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub startup: bool,
    pub actions: Vec<Action>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub hotkey: String,
    pub exec: String,
}

impl Config {}

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
}
