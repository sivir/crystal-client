// using deno since supabase uses deno
import { serve } from 'https://deno.land/std@0.177.0/http/server.ts'
import { update_db, cors_headers, get_user } from '../../shared/update_db.ts';

const riot_api_key = Deno.env.get('RIOT_API_KEY')!

serve(async (req) => {
	if (req.method === 'OPTIONS') {
		return new Response('ok', { headers: cors_headers });
	};

	const x = await req.json();
	const { id } = x;

	try {
		const res = await get_user(id);
		console.log("res", res);
		if (res.length === 0) {
			const response = await fetch(`https://na1.api.riotgames.com/lol/challenges/v1/player-data/${id}?api_key=${riot_api_key}`);
			const data = await response.json();
			console.log("data", data);
			await update_db(id, data);
			return new Response(data, { headers: cors_headers });
		} else {
			return new Response(res[0].test_data, { headers: cors_headers });
		}
	} catch (err) {
		console.error(err)
		return new Response(String(err?.message ?? err), { status: 500, headers: cors_headers });
	}
});