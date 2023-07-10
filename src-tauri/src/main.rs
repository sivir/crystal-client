// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, time};
use std::process;

use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{Manager, SystemTray};

use reqwest::header::AUTHORIZATION;

use futures::SinkExt;
use tokio_tungstenite::tungstenite::{protocol::WebSocketConfig, client::IntoClientRequest};
use futures_util::{StreamExt};
use http::{header, HeaderValue};
use reqwest::Response;
use tungstenite::{Error, Message};

#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_file(path: &str) -> Result<String, String> {
	return fs::read_to_string(path).map_err(|err| err.to_string());
}

#[tauri::command]
async fn http_request(url: &str, auth: &str) -> Result<String, String> {
	let client = reqwest::Client::builder()
		.danger_accept_invalid_certs(true)
		.build()
		.unwrap();
	loop {
		println!("trying 1");
		let request = client
			.get(url)
			.header(AUTHORIZATION, "Basic ".to_owned() + auth);
		match request.send().await {
			Ok(p) => {return p.text().await.map_err(|err| err.to_string());}
			Err(_) => {std::thread::sleep(time::Duration::from_millis(1000))}
		};
	}
}

#[tauri::command]
async fn start_lcu_websocket(port: &str, auth: &str /*endpoints: &[String]*/) -> Result<(), String> {
	loop {
		println!("happening 2");
		let go_to_connector = native_tls::TlsConnector::builder()
			.danger_accept_invalid_certs(true).build().unwrap();
		let connector = tokio_tungstenite::Connector::NativeTls(go_to_connector);
		let mut request = (port).into_client_request().unwrap();
		request.headers_mut().insert(AUTHORIZATION, auth.parse().unwrap());
		match tokio_tungstenite::connect_async_tls_with_config(request,
		                                                       Some(WebSocketConfig::default()), false,
		                                                       Some(connector)).await {
			Ok(a) => {
				let (mut socket, _) = a;
				println!("Connected");

				let message = "[5, \"OnJsonApiEvent\"]";
				socket.send(Message::Text(message.to_owned())).await.unwrap();
				'outer: loop {
					while let Some(msg) = socket.next().await {
						match msg {
							Ok(_) => {
								let msg = msg.unwrap();
								if msg.is_text() || msg.is_binary() {
									println!("{}",msg);
								}
							}
							Err(_) => { break 'outer }
						}

					}
				}
				return Ok(());
			}
			Err(_) => {std::thread::sleep(time::Duration::from_millis(1000))}
		}
	}
}

fn main() {
	let quit = CustomMenuItem::new("quit".to_string(), "Quit");
	let hide = CustomMenuItem::new("hide".to_string(), "Hide");
	let tray_menu = SystemTrayMenu::new()
		.add_item(quit)
		.add_native_item(SystemTrayMenuItem::Separator)
		.add_item(hide);

	let tray = SystemTray::new().with_menu(tray_menu);

	tauri::Builder::default()
		.system_tray(tray)
		.invoke_handler(tauri::generate_handler![greet, read_file, http_request, start_lcu_websocket])
		.plugin(tauri_plugin_fs_watch::init())
		.on_system_tray_event(|app, event| match event {
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
			SystemTrayEvent::MenuItemClick { id, .. } => {
				let item_handle = app.tray_handle().get_item(&id);
				match id.as_str() {
					"quit" => {
						process::exit(0);
					}
					"hide" => {
						let window = app.get_window("main").unwrap();
						if window.is_visible().unwrap() {
							window.hide().unwrap();
							item_handle.set_title("Show").unwrap();
						} else {
							window.show().unwrap();
							item_handle.set_title("Hide").unwrap();
						}
					}
					_ => {}
				}
			}
			_ => {}
		})
		.on_window_event(|event| match event.event() {
			tauri::WindowEvent::CloseRequested { api, .. } => {
				event.window().hide().unwrap();
				let item = event.window().app_handle().tray_handle().get_item("hide");
				item.set_title("Show").unwrap();
				api.prevent_close();
			}
			_ => {}
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}