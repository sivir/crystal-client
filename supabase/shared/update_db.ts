// using deno since supabase uses deno
import * as postgres from 'https://deno.land/x/postgres@v0.14.2/mod.ts'

const database_url = Deno.env.get('SUPABASE_DB_URL')!
const riot_api_key = Deno.env.get('RIOT_API_KEY')!

const pool = new postgres.Pool(database_url, 3, true);

const cors_headers = {
	'Access-Control-Allow-Origin': '*',
	'Access-Control-Allow-Headers': 'authorization, x-client-info, apikey, content-type',
};

async function update_riot_data(id: string) {
    const response = await fetch(`https://na1.api.riotgames.com/lol/challenges/v1/player-data/${id}?api_key=${riot_api_key}`);
    const data = await response.json();
    await update_db_riot_data(id, data);
    return data;
}

async function update_db_riot_data(id: string, data: any) {
    const connection = await pool.connect();
    await connection.queryObject`INSERT INTO users (id, riot_data) VALUES (${id}, ${data}) ON CONFLICT (id) DO UPDATE SET riot_data = ${data}`;
	connection.release();
}

async function update_db_lcu_data(id: string, data: any) {
    const connection = await pool.connect();
    await connection.queryObject`INSERT INTO users (id, lcu_data) VALUES (${id}, ${data}) ON CONFLICT (id) DO UPDATE SET lcu_data = ${data}`;
	connection.release();
}

async function get_user(id: string) {
    const connection = await pool.connect();
    const res = await connection.queryObject`SELECT * FROM test WHERE id = ${id}`;
    connection.release();
    return res.rows;
}

export { cors_headers, update_db_riot_data, get_user, update_riot_data };