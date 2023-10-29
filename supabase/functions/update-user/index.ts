// using deno since supabase uses deno
import * as postgres from 'https://deno.land/x/postgres@v0.14.2/mod.ts';
import { serve } from 'https://deno.land/std@0.177.0/http/server.ts';
import { update_db_lcu_data, cors_headers } from '../shared/update_db.ts';

serve(async (req) => {
	if (req.method === 'OPTIONS') {
		return new Response('ok', { headers: cors_headers });
	}

	const x = await req.json();
	const { id, data } = x;

	try {
		update_db_lcu_data(id, data);
		return new Response('ok', { headers: cors_headers });
	} catch (err) {
		console.error(err);
		return new Response(String(err?.message ?? err), { status: 500, headers: cors_headers });
	}
});