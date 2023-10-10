use std::time::Duration;

use tao::event_loop::{EventLoopBuilder, ControlFlow};
use tray_icon::{menu::{Menu, MenuItem, MenuEvent}, TrayIconBuilder, Icon, TrayIconEvent, ClickType};
use anyhow::Result;

use crate::open_app;

const ICON: &[u8] = include_bytes!("../icon.png");

pub fn main() -> Result<()>{
    let icon = load_icon()?;

    let event_loop = EventLoopBuilder::new().build();

    let menu = Menu::new();
    menu.append(&MenuItem::new("打开", true, None))?;
    menu.append(&MenuItem::new("退出", true, None))?;

    let _tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("多窗口")
            .with_icon(icon)
            .build()?,
    );

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();
    
    let event_loop_proxy = event_loop.create_proxy();
    std::thread::spawn(move || {
        loop {
            event_loop_proxy.send_event(()).ok();
            std::thread::sleep(Duration::from_millis(50));
        }
    });

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(MenuEvent { id }) = menu_channel.try_recv() {
            if id.0 == "1001"{
                //打开app
                open_app();
            }else{
                //退出
                *control_flow = ControlFlow::Exit;
            }
        }
        
        if let Ok(TrayIconEvent {click_type, id: _, x: _, y: _, icon_rect: _ }) = tray_channel.try_recv(){
            if let ClickType::Left = click_type{
                //打开app
                open_app();
            }
        }
    });
}

fn load_icon() -> Result<Icon>{
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory_with_format(ICON, image::ImageFormat::Png)?.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Ok(tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)?)
}