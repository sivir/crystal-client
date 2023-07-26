use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use futures::{SinkExt, channel::mpsc::{channel, Receiver}};
use tauri::{AppHandle, Manager};
use futures_util::StreamExt;
use std::path::Path;

use crate::data;
use data::Data;

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
pub async fn async_watch(state: tauri::State<'_, Data>, app_handle: AppHandle) -> Result<(), ()> {
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