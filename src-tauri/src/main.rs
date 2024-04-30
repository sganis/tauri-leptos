// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CmdResult {
    cmd: String,
    stdout: String,
    stderr: String,
    retcode: i32,
}

// #[tauri::command]
// fn greet(param: &str) -> String {
//     println!("running: {}", param);
//     let (o, e, r) = command::run(param);
//     format!("hello {}", o)
// }

#[tauri::command]
fn run_local(param: &str) -> Result<CmdResult, String> {
    println!("running: {}", param);
    let (o, e, r) = command::run(param);

    let result = CmdResult {
        cmd: param.to_string(),
        stdout: o,
        stderr: e,
        retcode: r,
    };
    println!("result: {:?}", result.clone());
    if result.stderr != "" {
        Err(result.stderr)
    } else {
        Ok(result)
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_local])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
