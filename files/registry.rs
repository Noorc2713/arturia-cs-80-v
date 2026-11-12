//! Windows registry read helpers for Arturia Cs 80 V.

pub const APP_KEY: &str = "Software\\Arturia\\Arturia Cs 80 V";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Arturia Cs 80 V";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Arturia Cs 80 V") },
        RegValue { name: "Version",     value: String::from("v2.1.15") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Arturia Cs 80 V") },
        RegValue { name: "Publisher",       value: String::from("Arturia") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Arturia Cs 80 V\\uninstall.exe\"") },
    ]
}
