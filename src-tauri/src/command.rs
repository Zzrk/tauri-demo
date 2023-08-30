// 主进程状态结构体
pub struct Database;

// 响应结构体, 必须实现 serde::Serialize
#[derive(serde::Serialize)]
pub struct CustomResponse {
  message: String,
  other_val: String,
}

// 异步任务
async fn some_other_function() -> Option<String> {
  Some("response".into())
}

#[tauri::command]
pub async fn my_custom_command(
  window: tauri::Window, // 调用消息的 Window 实例
  name: &str, // 渲染进程传参
  _database: tauri::State<'_, Database>, // 主进程状态
) -> Result<CustomResponse, String> {
  println!("Called from {}", window.label());
  let result: Option<String> = some_other_function().await;
  if let Some(message) = result {
    Ok(CustomResponse {
      message,
      other_val: format!("Hello {name}!"),
    })
  } else {
    Err("No result".into())
  }
}