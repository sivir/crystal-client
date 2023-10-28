// using deno since supabase uses deno
import { serve } from 'https://deno.land/std@0.177.0/http/server.ts';
import { update_db, cors_headers, get_user, update_riot_data } from '../shared/update_db.ts';

const riot_api_key = Deno.env.get('RIOT_API_KEY')!

serve(async (req) => {
	// allow calling from browser
	if (req.method === 'OPTIONS') {
		return new Response('ok', { headers: cors_headers });
	}

	try {
		const data = await fetch(`https://na1.api.riotgames.com/lol/challenges/v1/challenges/config?api_key=${riot_api_key}`);
		const challenge_data = await data.json();
		console.log(challenge_data);
        return new Response(JSON.stringify(challenge_data), { headers: cors_headers });
    } catch (err) {
		console.error(err);
		return new Response(String(err?.message ?? err), { status: 500, headers: cors_headers });
	}
});