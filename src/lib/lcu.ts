import {invoke} from "@tauri-apps/api/tauri"

export function get_endpoint(endpoint: string) {
	invoke("http_request").then(x => x).catch(x => console.error(x));
}