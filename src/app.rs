use std::time::Duration;

use anyhow::Result;
use ipc_channel::ipc::IpcOneShotServer;
use serde::{Serialize, Deserialize};
use crate::open_dialog;

#[derive(Serialize, Deserialize, Debug)]
pub enum IpcMessage{
    LabelMessage(String)
}

slint::slint!{
    import { Button , HorizontalBox, VerticalBox} from "std-widgets.slint";
    export component App inherits Window {
        title: "主窗口";
        width: 320px;
        height: 240px;
        icon: @image-url("icon.png");

        callback open-dialog();
        in-out property <string> label-text: "hello world!";
        in-out property <string> ipc-server-name: "";
        
        VerticalBox {
            Button { 
                text: "打开对话框";
                width: 100px;
                height: 40px;
                clicked => {
                    open-dialog()
                }
            }
            Text {
                text: label-text;
                color: green;
            }
        }
    }
}

pub fn main() -> Result<()>{
    let app = App::new()?;

    //接收其他窗口传递的消息
    start_ipc_server(&app)?;

    let app_clone = app.as_weak();
    app.on_open_dialog(move ||{
        let app = app_clone.unwrap();
        let pos = app.window().position();
        let _ = open_dialog(app.get_ipc_server_name().to_string(), pos.x, pos.y);
    });

    app.run()?;
    Ok(())
}

fn start_ipc_server(app: &App) -> Result<()>{
    let (server, name):(IpcOneShotServer<IpcMessage>, String) = IpcOneShotServer::new()?;
    app.set_ipc_server_name(name.into());
    let app_clone = app.as_weak();
    std::thread::spawn(move ||{
        let (rx, mut data) = server.accept().unwrap();
        loop{
            match &data {
                IpcMessage::LabelMessage(label_text) => {
                    let label_text = label_text.to_string();
                    let app_clone = app_clone.clone();
                    slint::invoke_from_event_loop(move || app_clone.unwrap().set_label_text(label_text.into())).unwrap();
                },
            }
            if let Ok(d) = rx.try_recv(){
                data = d;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    });
    Ok(())
}