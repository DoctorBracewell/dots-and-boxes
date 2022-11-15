import type { FastifyInstance } from "fastify";
import Surreal from "surrealdb.js";
import { basename } from "path";

interface Body {
	username: string;
}

interface Querystring {
	name: string;
}

const route = basename(import.meta.url, ".js");

export function setupRoutes(server: FastifyInstance) {
	server.post<{
		Body: Body;
	}>(`/${route}`, async (request, reply) => {
		const { username } = request.body;

		await Surreal.Instance.query(
			"CREATE game SET name = $username, game_time=time::now()",
			{
				username,
			}
		);
	});

	server.get<{
		Querystring: Querystring;
	}>(`/${route}`, async (request, reply) => {
		const { name } = request.query;

		const user = await Surreal.Instance.query(
			"SELECT * FROM game WHERE name = $name",
			{
				name,
			}
		);

		return user;
	});
}
