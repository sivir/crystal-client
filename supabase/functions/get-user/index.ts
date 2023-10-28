// using deno since supabase uses deno
import { serve } from 'https://deno.land/std@0.177.0/http/server.ts'
import { update_db, cors_headers, get_user } from '../shared/update_db.ts';

const riot_api_key = Deno.env.get('RIOT_API_KEY')!

const region = "na1"; // will change

serve(async (req) => {
	// allow calling from browser
	if (req.method === 'OPTIONS') {
		return new Response('ok', { headers: cors_headers });
	}

	// extract id from request
	const x = await req.json();
	const { summoner_name } = x;

	try {
		// check if user exists in db
		const summoner_response = await fetch(`https://${region}.api.riotgames.com/lol/summoner/v4/summoners/by-name/${summoner_name}?api_key=${riot_api_key}`);
		const summoner_data = await summoner_response.json();
		const id = summoner_data.puuid;
		console.log("summoner_data", summoner_data);
		const res = await get_user(id);
		console.log("res", res);
		// if not, update db with riot data
		if (res.length === 0) {
			const data = await update_riot_id(id);
			return new Response(JSON.stringify({riot_data: data, lcu_data: {}}), { headers: cors_headers });
		} else {
			// check when riot data was last updated
			const last_updated = new Date(res[0].last_update_riot);
			const now = new Date();
			const diff = now.getTime() - last_updated.getTime();
			// if it's been 10 minutes, update it
			if (diff > 10 * 60 * 1000) {
				const data = await update_riot_id(id);
				return new Response(JSON.stringify({riot_data: data, lcu_data: res[0].lcu_data}), { headers: cors_headers });
			} else {
				// otherwise, return it
				return new Response(JSON.stringify({riot_data: res[0].riot_data, lcu_data: res[0].lcu_data}), { headers: cors_headers });
			}
		}
	} catch (err) {
		console.error(err);
		return new Response(String(err?.message ?? err), { status: 500, headers: cors_headers });
	}
});