// using deno since supabase uses deno
import * as postgres from 'https://deno.land/x/postgres@v0.14.2/mod.ts'
import { serve } from 'https://deno.land/std@0.177.0/http/server.ts'

const database_url = Deno.env.get('SUPABASE_DB_URL')!
const pool = new postgres.Pool(database_url, 3, true);

console.log("Hello from Functions!");

const corsHeaders = {
	'Access-Control-Allow-Origin': '*',
	'Access-Control-Allow-Headers': 'authorization, x-client-info, apikey, content-type',
};

serve(async (req) => {
	if (req.method === 'OPTIONS') {
		return new Response('ok', { headers: corsHeaders });
	};
	const x = await req.json();
	const { name } = x;

	try {
		const connection = await pool.connect();
		const result = await connection.queryObject`SELECT * FROM test WHERE id = ${name}`;
		const animals = result.rows;
		console.log(animals);

		connection.release();
		return new Response(JSON.stringify(animals),
			{
				headers: {
					...corsHeaders,
					"Content-Type": "application/json"
				}
			},
		);
	} catch (err) {
		console.error(err)
		return new Response(String(err?.message ?? err), { status: 500, headers: corsHeaders });
	}
});

// ./curl -L -X POST 'https://jvnhtmgsncslprdrnkth.supabase.co/functions/v1/hello-world' -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Imp2bmh0bWdzbmNzbHByZHJua3RoIiwicm9sZSI6ImFub24iLCJpYXQiOjE2OTQ2Mjc4ODMsImV4cCI6MjAxMDIwMzg4M30.OOjwsPjGHEc-x8MlhrOX64tJTNENqKqEq2635HKErrk' --data '{\"name\":\"Functions\"}'