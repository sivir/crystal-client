use serde_json::Value;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct InnerData {
	pub lockfile: bool, // whether the lockfile exists
	pub install_dir: String, // installation directory of the game (folder name including end backslash)
	pub port: String, // port for websocket and http requests
	pub auth: String, // auth string, still requires "Basic" added to it for header auth
	pub challenge_data: Value, // challenge completion/tier data
	pub champion_data: Value, // champion mastery data
	pub summoner_id: String, // ppuid of current player
}

impl Default for InnerData {
	fn default() -> Self {
		InnerData {
			lockfile: false,
			install_dir: "C:\\Riot Games\\League of Legends\\".to_string(),
			port: "".to_string(),
			auth: "".to_string(),
			challenge_data: Value::Null,
			champion_data: Value::Null,
			summoner_id: "".to_string(),
		}
	}
}

pub struct Data(pub Mutex<InnerData>);

