// using deno since supabase uses deno
import { serve } from 'https://deno.land/std@0.177.0/http/server.ts'
import { update_db, cors_headers, get_user } from '../shared/update_db.ts';

const riot_api_key = Deno.env.get('RIOT_API_KEY')!

const region = "na1"; // will change

serve(async (req) => {
	if (req.method === 'OPTIONS') {
		return new Response('ok', { headers: cors_headers });
	};

	const x = await req.json();
	const { summoner_name } = x;

	try {
		const summoner_response = await fetch(`https://${region}.api.riotgames.com/lol/summoner/v4/summoners/by-name/${summoner_name}?api_key=${riot_api_key}`);
		const summoner_data = await summoner_response.json();
		const id = summoner_data.puuid;
		console.log(id, "id");
		const res = await get_user(id);
		if (res.length === 0) {
			const response = await fetch(`https://${region}.api.riotgames.com/lol/challenges/v1/player-data/${id}?api_key=${riot_api_key}`);
			const data = await response.json();
			console.log("data", data);
			await update_db(id, data);
			return new Response(JSON.stringify(data), { headers: cors_headers });
		} else {
			return new Response(JSON.stringify(res[0].test_data), { headers: cors_headers });
		}
	} catch (err) {
		console.error(err)
		return new Response(String(err?.message ?? err), { status: 500, headers: cors_headers });
	}
});