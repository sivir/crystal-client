// using deno since supabase uses deno
import * as postgres from 'https://deno.land/x/postgres@v0.14.2/mod.ts'

const database_url = Deno.env.get('SUPABASE_DB_URL')!
const pool = new postgres.Pool(database_url, 3, true);

const cors_headers = {
	'Access-Control-Allow-Origin': '*',
	'Access-Control-Allow-Headers': 'authorization, x-client-info, apikey, content-type',
};

async function update_db(id: string, data: any) {
    const connection = await pool.connect();
    await connection.queryObject`INSERT INTO test (id, test_data) VALUES (${id}, ${data}) ON CONFLICT (id) DO UPDATE SET test_data = ${data}`;
	connection.release();
}

async function get_user(id: string) {
    const connection = await pool.connect();
    const res = await connection.queryObject`SELECT * FROM test WHERE id = ${id}`;
    connection.release();
    return res.rows;
}

export { cors_headers, update_db, get_user };