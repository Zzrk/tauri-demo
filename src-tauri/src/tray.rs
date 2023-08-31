use tauri::{
  CustomMenuItem,
  SystemTrayMenu,
  SystemTrayMenuItem,
  SystemTray,
  AppHandle,
  SystemTrayEvent,
  Manager
};

// 初始化系统托盘
pub fn init_tray() -> SystemTray {
  // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide);

  // 添加菜单的托盘
  SystemTray::new().with_menu(tray_menu)
}

// 处理托盘事件
pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
  match event {
    SystemTrayEvent::LeftClick {
      position: _,
      size: _,
      ..
    } => {
      println!("system tray received a left click");
    }
    SystemTrayEvent::RightClick {
      position: _,
      size: _,
      ..
    } => {
      println!("system tray received a right click");
    }
    SystemTrayEvent::DoubleClick {
      position: _,
      size: _,
      ..
    } => {
      println!("system tray received a double click");
    }
    // 托盘菜单
    SystemTrayEvent::MenuItemClick { id, .. } => {
      let item_handle = app.tray_handle().get_item(&id);
      match id.as_str() {
        "quit" => {
          std::process::exit(0);
        }
        "hide" => {
          let window = app.get_window("main").unwrap();
          window.hide().unwrap();
          item_handle.set_title("Show").unwrap();
        }
        _ => {}
      }
    }
    _ => {}
  }
}