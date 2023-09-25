// using deno since supabase uses deno

import * as postgres from 'https://deno.land/x/postgres@v0.14.2/mod.ts'
import { serve } from 'https://deno.land/std@0.177.0/http/server.ts'

// Get the connection string from the environment variable "SUPABASE_DB_URL"
const databaseUrl = Deno.env.get('SUPABASE_DB_URL')!

// Create a database pool with three connections that are lazily established
const pool = new postgres.Pool(databaseUrl, 3, true)

console.log("Hello from Functions!")

const corsHeaders = {
	'Access-Control-Allow-Origin': '*',
	'Access-Control-Allow-Headers': 'authorization, x-client-info, apikey, content-type',
}

serve(async (req) => {
	if (req.method === 'OPTIONS') {
		return new Response('ok', { headers: corsHeaders })
	}
	const x = await req.json();
	console.log("name", x);
	const { name } = x;
	const data = {
		message: `Hello ${name} ${databaseUrl}!`,
	}

	try {
		const connection = await pool.connect();
		const result = await connection.queryObject`SELECT * FROM test WHERE id = ${name}`
		const animals = result.rows // [{ id: 1, name: "Lion" }, ...]
		console.log(animals)

		connection.release();
		return new Response(
			JSON.stringify(animals),
			{ headers: {
				...corsHeaders,
				 "Content-Type": "application/json" } },
		)
	} catch (err) {
		console.error(err)
		return new Response(String(err?.message ?? err), { status: 500,  headers: corsHeaders });
	  }

	
})

// To invoke:
// curl -i --location --request POST 'http://localhost:54321/functions/v1/' \
//   --header 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0' \
//   --header 'Content-Type: application/json' \
//   --data '{"name":"Functions"}'
