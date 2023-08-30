use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// 初始化菜单
pub fn init_menu() -> Menu {
  // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
  Menu::new()
    .add_native_item(MenuItem::Copy) //  MenuItem 枚举包含一系列特定平台的项（目前在 Windows 上没有实现）
    .add_item(CustomMenuItem::new("hide", "Hide")) // 自定义菜单项
    .add_submenu(submenu) // 二级菜单
}

// 处理菜单事件
pub fn handle_menu_event(event: tauri::WindowMenuEvent) {
  match event.menu_item_id() {
    "quit" => {
      std::process::exit(0);
    }
    "close" => {
      event.window().close().unwrap();
    }
    _ => {}
  }
}