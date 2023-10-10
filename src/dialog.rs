use anyhow::Result;
use ipc_channel::ipc::IpcSender;
use slint::PhysicalPosition;

use crate::app::IpcMessage;

slint::slint!{
    import { Button , HorizontalBox, VerticalBox} from "std-widgets.slint";
    export component Dialog inherits Window {
        title: "对话框";
        width: 300px;
        height: 200px;
        icon: @image-url("icon.png");

        callback close-dialog();
        
        VerticalBox {
            Text {
                text: "对话框";
                color: green;
            }
            Button { 
                text: "返回数据并关闭";
                width: 120px;
                height: 40px;
                clicked => {
                    close-dialog()
                }
            }
        }
    }
}

pub fn main(ipc_server_name:String, x: i32, y:i32) -> Result<()>{
    let dialog = Dialog::new()?;

    dialog.window().set_position(PhysicalPosition::new(x, y));

    let dialog_handle = dialog.as_weak();
    dialog.on_close_dialog(move ||{
        let server_name = ipc_server_name.clone();
        //给主窗口返回数据
        let tx: IpcSender<IpcMessage> = IpcSender::connect(server_name).unwrap();
        tx.send(IpcMessage::LabelMessage("数据更新成功!".to_string())).unwrap();
        dialog_handle.unwrap().hide().unwrap();
    });

    dialog.run()?;
    Ok(())
}