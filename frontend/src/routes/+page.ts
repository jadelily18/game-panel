import type { PageLoad } from './$types';
import { env } from '$env/dynamic/public';

export const load: PageLoad = async ({ fetch }) => {
	// let rand_res = await fetch('http://backend:8080/rand');
	let rand_res = await fetch(`${env.PUBLIC_BACKEND_URL}/rand`);

	let rand_number = await rand_res.text();

	return { number: rand_number };
};
