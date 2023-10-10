#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use anyhow::Result;

mod app;
mod dialog;
mod tray_icon;

pub const ARGS_APP:&str = "app";
pub const ARGS_DIALOG:&str = "dialog";

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1{
        let arg1 = args[1].to_lowercase();
        if arg1.starts_with(ARGS_APP) {
            //打开App
            return app::main();
        }else if arg1.starts_with(ARGS_DIALOG){
            let ipc_server_name = args[4].to_string();
            //打开对话框
            return dialog::main(ipc_server_name, args[2].parse()?, args[3].parse()?);
        }
    }

    //打开app
    open_app();

    //打开图标
    tray_icon::main()
}

pub fn open_app(){
    let _ = start_process(vec![ARGS_APP.to_string()]);
}

pub fn open_dialog(ipc_server_name: String, x: i32, y:i32){
    let _ = start_process(vec![ARGS_DIALOG.to_string(), format!("{x}"), format!("{y}"), ipc_server_name]);
}

fn start_process(command_args: Vec<String>) -> Result<()>{
    // 获取当前可执行文件的路径
    let current_exe = std::env::current_exe()?;

    // 启动新进程并传递命令行参数
    Command::new(current_exe)
        .args(&command_args)
        .spawn()?;
    Ok(())
}
