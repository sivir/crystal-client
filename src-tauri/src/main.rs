// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, process, time};

use base64::{Engine as _, engine::general_purpose};
use futures::SinkExt;
use futures_util::StreamExt;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use tauri::{AppHandle, CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{Manager, SystemTray};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, protocol::WebSocketConfig};
use tungstenite::Message;

use data::Data;

mod file_watcher;
mod data;

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
	let lockfile_dir = format!("{}lockfile", data.install_dir);
	match fs::read_to_string(lockfile_dir) {
		Ok(raw_contents) => {
			data.lockfile = true;
			app.emit_all("lockfile", "create").unwrap();
			println!("file found");
			let contents = raw_contents.split(":").collect::<Vec<_>>();
			data.port = contents[2].to_string();
			let auth = format!("riot:{}", contents[3]);
			data.auth = general_purpose::STANDARD.encode(auth);
		}
		Err(_) => {
			data.lockfile = false;
			app.emit_all("lockfile", "remove").unwrap();
			println!("file not found");
		}
	};
	Ok(())
}

#[tauri::command]
async fn update_challenge_data(state: tauri::State<'_, Data>) -> Result<(), ()> {
	let res = http_retry("lol-challenges/v1/challenges/local-player", state.clone()).await.unwrap();
	let mut data = state.0.lock().await;
	data.challenge_data = res;

	Ok(())
}

#[tauri::command]
async fn update_all_data(app_handle: AppHandle) -> Result<(), ()> {
	println!("update all data");
	update_summoner_id(app_handle.state()).await.unwrap();
	update_champion_data(app_handle.state()).await.unwrap();
	update_challenge_data(app_handle.state()).await.unwrap();

	Ok(())
}

#[tauri::command]
async fn update_summoner_id(state: tauri::State<'_, Data>) -> Result<(), ()> {
	let res = http_retry("lol-summoner/v1/current-summoner", state.clone()).await.unwrap();
	let mut data = state.0.lock().await;
	data.summoner_id = res["summonerId"].to_string();

	Ok(())
}

#[tauri::command]
async fn update_champion_data(state: tauri::State<'_, Data>) -> Result<(), ()> {
	let summoner_id = state.0.lock().await.summoner_id.clone();
	let endpoint = format!("lol-collections/v1/inventories/{summoner_id}/champion-mastery");
	let res = http_retry(endpoint.as_str(), state.clone()).await.unwrap();
	let mut data = state.0.lock().await;
	data.champion_data = res;

	Ok(())
}

#[tauri::command]
async fn get_challenge_data(state: tauri::State<'_, Data>) -> Result<Value, ()> {
	let data = state.0.lock().await;
	let challenge_data = data.challenge_data.clone();
	Ok(challenge_data)
}

#[tauri::command]
async fn get_champion_data(state: tauri::State<'_, Data>) -> Result<Value, ()> {
	let data = state.0.lock().await;
	let champion_data = data.champion_data.clone();
	Ok(champion_data)
}

#[tauri::command]
async fn http_generic(url: &str) -> Result<Value, ()> {
	let client = reqwest::Client::builder().build().unwrap();
	return Ok(client.get(url).send().await.unwrap().json::<Value>().await.unwrap());
}

#[tauri::command]
async fn get_champion_map() -> Value {
	let version_list = http_generic("https://ddragon.leagueoflegends.com/api/versions.json").await.unwrap();
	let version = version_list[0].as_str().unwrap();
	// println!("{}", version);
	let champ_url = format!("https://ddragon.leagueoflegends.com/cdn/{version}/data/en_US/champion.json");
	let c = http_generic(champ_url.as_str()).await.unwrap();
	return c;
}

#[tauri::command]
async fn http_retry(endpoint: &str, state: tauri::State<'_, Data>) -> Result<Value, String> {
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
			Ok(response) => {
				let json = response.json::<Value>().await.unwrap();
				if json["httpStatus"].is_number() {
					std::thread::sleep(time::Duration::from_millis(1000));
				} else {
					return Ok(json)
				}
			},
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
									let json: Value = serde_json::from_str(msg.to_string().as_str()).unwrap();
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
		.manage(Data(Mutex::new(data::InnerData::default())))
		.system_tray(tray)
		.invoke_handler(tauri::generate_handler![
			get_challenge_data,
			get_champion_data,
			update_all_data,
			update_challenge_data,
			read_file,
			http_retry,
			start_lcu_websocket,
			process_lockfile,
			file_watcher::async_watch,
			http_generic,
			get_champion_map
		])
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