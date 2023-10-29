// using deno since supabase uses deno
import * as postgres from 'https://deno.land/x/postgres@v0.14.2/mod.ts';

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
    const time = new Date();
    await connection.queryObject`INSERT INTO users (id, riot_data, last_update_riot) VALUES (${id}, ${data}, ${time}) ON CONFLICT (id) DO UPDATE SET (riot_data, last_update_riot) = (${data}, ${time})`;
	connection.release();
}

async function update_db_lcu_data(id: string, data: any) {
    const connection = await pool.connect();
    const user = await get_user(id);
    if (user.length === 0) {
        await update_riot_data(id);
    }
    const time = new Date();
    // update lcu data and time for user without inserting
    await connection.queryObject`UPDATE users SET (lcu_data, last_update_lcu) = (${data}, ${time}) WHERE id = ${id}`;
	connection.release();
}

async function get_user(id: string) {
    const connection = await pool.connect();
    const res = await connection.queryObject`SELECT * FROM users WHERE id = ${id}`;
    console.log("get_user res", res);
    connection.release();
    return res.rows;
}

export { cors_headers, update_db_riot_data, get_user, update_riot_data, update_db_lcu_data };