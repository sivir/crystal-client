// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, time, process, path::Path};
use tokio::sync::Mutex;

use tauri::{AppHandle, CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{Manager, SystemTray};

use reqwest::header::AUTHORIZATION;

use tokio_tungstenite::tungstenite::{protocol::WebSocketConfig, client::IntoClientRequest};
use futures_util::StreamExt;
use tungstenite::Message;
use base64::{Engine as _, engine::general_purpose};
use futures::{SinkExt, channel::mpsc::{channel, Receiver}};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde_json::Value;

mod greet;

#[tauri::command]
async fn read_file(state : tauri::State<'_, Data>) -> Result<String, String> {
	let data = state.0.lock().await;
	let path = data.install_dir.clone();
	drop(data);
	return fs::read_to_string(format!("{}lockfile", path)).map_err(|err| err.to_string());
}

#[tauri::command]
async fn process_lockfile(app: AppHandle, state: tauri::State<'_, Data>) -> Result<(), ()> {
	let mut data = state.0.lock().await;
	println!("pro");
	let lockfile_dir = format!("{}lockfile", data.install_dir);
	println!("{}", lockfile_dir);
	match fs::read_to_string(lockfile_dir) {
		Ok(raw_contents) => {
			data.lockfile = true;
			app.emit_all("lockfile", "create").unwrap();
			println!("filel found");
			let contents = raw_contents.split(":").collect::<Vec<_>>();
			data.port = contents[2].to_string();
			let auth = format!("riot:{}", contents[3]);
			data.auth = general_purpose::STANDARD.encode(auth);
			println!("{:?}", data);
		}
		Err(_) => {
			data.lockfile = false;
			app.emit_all("lockfile", "remove").unwrap();
			println!("file not found");
		}
	};
	Ok(())
}

#[derive(Debug)]
struct InnerData {
	lockfile : bool, // whether the lockfile exists
	install_dir : String, // installation directory of the game (folder name including end backslash)
	port : String, // port for websocket and http requests
	auth : String, // auth string, still requires "Basic" added to it for header auth
	challenge_data: Value
}

impl Default for InnerData {
	fn default() -> Self {
		InnerData {
			lockfile: false,
			install_dir: "C:\\Riot Games\\League of Legends\\".to_string(),
			port: "".to_string(),
			auth: "".to_string(),
			challenge_data: Value::Null
		}
	}
}

struct Data(Mutex<InnerData>);

#[tauri::command]
async fn update_challenge_data(state: tauri::State<'_, Data>) -> Result<(), ()> {
	let state1 = state.clone();
	let res = http_retry("lol", state1).await.unwrap();
	let mut data = state.0.lock().await;
	data.challenge_data = serde_json::from_str(res.as_str()).unwrap();


	Ok(())
}

#[tauri::command]
async fn get_challenge_data(state: tauri::State<'_, Data>) -> Result<Value, String> {
	let data = state.0.lock().await;
	let challenge_data = data.challenge_data.clone();
	return Ok(challenge_data)
}

#[tauri::command]
async fn http_retry(endpoint: &str, state: tauri::State<'_, Data>) -> Result<String, String> {
	let data = state.0.lock().await;
	let port = data.port.clone();
	let auth = data.auth.clone();
	drop(data);
	let url = format!("https://127.0.0.1:{port}/{endpoint}");

	let client = reqwest::Client::builder()
		.danger_accept_invalid_certs(true)
		.build()
		.unwrap();
	loop {
		let request = client
			.get(url.as_str())
			.header(AUTHORIZATION, format!("Basic {auth}"));
		match request.send().await {
			Ok(response) => return response.text().await.map_err(|err| err.to_string()),
			Err(_) => std::thread::sleep(time::Duration::from_millis(1000))
		};
	}
}

#[tauri::command]
async fn start_lcu_websocket(endpoints: Vec<&str>, app_handle: AppHandle, state: tauri::State<'_, Data>) -> Result<(), String> {
	let data = state.0.lock().await;
	let port = data.port.clone();
	let auth_string = data.auth.clone();
	drop(data);

	let auth = format!("Basic {auth_string}");
	let url = format!("wss://127.0.0.1:{port}/");
	loop {
		let tls_connector = native_tls::TlsConnector::builder()
			.danger_accept_invalid_certs(true).build().unwrap();
		let connector = tokio_tungstenite::Connector::NativeTls(tls_connector);
		let mut request = url.clone().into_client_request().unwrap();
		request.headers_mut().insert(AUTHORIZATION, auth.parse().unwrap());
		match tokio_tungstenite::connect_async_tls_with_config(request,
		                                                       Some(WebSocketConfig::default()),
		                                                       false,
		                                                       Some(connector)).await {
			Ok(connection_response) => {
				let (mut socket, _) = connection_response;
				println!("Connected");

				for endpoint in endpoints.iter() {
					let message = format!("[5, \"{}\"]", endpoint);
					socket.send(Message::Text(message.to_owned())).await.unwrap();
				}

				'outer: loop {
					while let Some(msg) = socket.next().await {
						match msg {
							Ok(_) => {
								let msg = msg.unwrap();
								if msg.is_text() && !msg.to_string().is_empty() {
									println!("|{msg}|");

									let json: Value = serde_json::from_str(msg.to_string().as_str()).unwrap();
									println!("{}", json[2]["data"]);
									app_handle.emit_all("gameflow", json[2]["data"].as_str()).unwrap();
								}
							}
							Err(_) => break 'outer
						}

					}
				}
				println!("disconnected!");
				return Ok(());
			}
			Err(_) => std::thread::sleep(time::Duration::from_millis(1000))
		}
	}
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
	let (mut tx, rx) = channel(1);

	let watcher = RecommendedWatcher::new(
		move |res| {
			futures::executor::block_on(async {
				tx.send(res).await.unwrap();
			})
		},
		Config::default(),
	)?;

	Ok((watcher, rx))
}

#[tauri::command]
async fn async_watch(state: tauri::State<'_, Data>, app_handle: AppHandle) -> Result<(), ()> {
	let data = state.0.lock().await;
	let path = data.install_dir.clone();
	drop(data);

	let (mut watcher, mut rx) = async_watcher().unwrap();

	watcher.watch(Path::new(&path), RecursiveMode::NonRecursive).unwrap();

	while let Some(res) = rx.next().await {
		match res {
			Ok(event) => {
				if format!("{}lockfile", path).eq(event.paths[0].to_str().unwrap()) {
					match event.kind {
						EventKind::Create(_) => {
							app_handle.emit_all("lockfile", "create").unwrap();
							let mut data = state.0.lock().await;
							data.lockfile = true;
							drop(data);
							println!("create!");
						}
						EventKind::Remove(_) => {
							println!("remove!");
							let mut data = state.0.lock().await;
							data.lockfile = false;
							drop(data);
							app_handle.emit_all("lockfile", "remove").unwrap();
						}
						_ => {}
					}
					println!("changed: {:?}", event)
				}
			},
			Err(e) => println!("watch error: {:?}", e),
		}
	}

	Ok(())
}

fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
	let window = app.get_window("main").unwrap();
	match event {
		SystemTrayEvent::LeftClick { .. } => {
			if !window.is_visible().unwrap() {
				window.show().unwrap();
				let item_handle = app.tray_handle().get_item("hide");
				item_handle.set_title("Hide").unwrap();
			}
		}
		SystemTrayEvent::MenuItemClick { id, .. } => {
			let item_handle = app.tray_handle().get_item(&id);
			match id.as_str() {
				"quit" => {
					process::exit(0);
				}
				"hide" => {
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
		.manage(Data(Mutex::new(InnerData::default())))
		.system_tray(tray)
		.invoke_handler(tauri::generate_handler![greet::greet, read_file, http_retry, start_lcu_websocket, process_lockfile, async_watch])
		.on_system_tray_event(handle_tray_event)
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