#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tokio::task::spawn_blocking(move ||{
    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| {
          use cmd::Cmd::*;
          match serde_json::from_str(arg) {
            Err(e) => {
              Err(e.to_string())
            }
            Ok(command) => {
              match command {
                // definitions for your custom commands from Cmd here
                MyCustomCommand { argument } => {
                  //  your command code
                  println!("{}", argument);
                }
              }
              Ok(())
            }
          }
        })
        .build()
        .run();
  }).await?;
   Ok(())
}
