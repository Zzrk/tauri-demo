// pub fn create_external_window_in_setup(app: &mut tauri::App) {
//   tauri::WindowBuilder::new(
//     app,
//     "external",
//     tauri::WindowUrl::External("https://tauri.app/".parse().unwrap())
//   ).build().expect("failed to build window");
// }

#[tauri::command]
pub async fn open_external_window(handle: tauri::AppHandle) {
  tauri::WindowBuilder::new(
    &handle,
    "external", /* the unique window label */
    tauri::WindowUrl::External("https://tauri.app/".parse().unwrap())
  ).build().unwrap();
}