// 关闭构建好的应用在 Windows 上运行时一般会出现的控制台窗口。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod menu;
mod window;

use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // 模拟初始化过程
      let splashscreen_window = app.get_window("splashscreen").unwrap();
      let main_window = app.get_window("main").unwrap();
      tauri::async_runtime::spawn(async move {
        println!("Initializing...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("Done initializing.");
        splashscreen_window.close().unwrap();
        main_window.show().unwrap();
      });

      // 创建外部链接的窗口
      // window::create_external_window_in_setup(app);
      Ok(())
    })
    .menu(menu::init_menu())
    .on_menu_event(menu::handle_menu_event)
    .manage(command::Database {})
    .invoke_handler(tauri::generate_handler![
      command::my_custom_command,
      window::open_external_window,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}