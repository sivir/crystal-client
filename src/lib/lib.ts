import {invoke} from "@tauri-apps/api/tauri";

export function http_request(endpoint: string, handler: (x: string) => void) {
	invoke("http_retry", {endpoint: endpoint}).then(handler);
}