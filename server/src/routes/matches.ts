import type { FastifyInstance } from "fastify";
import Surreal from "surrealdb.js";
import { basename } from "path";

interface Body {
	username: string;
	result: {
		player: number;
		opponent: number;
	};
	board: {
		width: number;
		height: number;
	};
}

interface Querystring {
	username?: string;
}

const route = basename(import.meta.url, ".js");

export function setupRoutes(server: FastifyInstance) {
	server.post<{
		Body: Body;
	}>(`/${route}`, async (request) => {
		const { username, result, board } = request.body;

		await Surreal.Instance.query(
			"CREATE game SET username = $username, result = $result, game_time = time::now()",
			{
				username,
				result,
				board,
			}
		);
	});

	server.get<{
		Querystring: Querystring;
	}>(`/${route}`, async (request) => {
		if (request.query.username ?? false) {
			const { username } = request.query;

			const matches = await Surreal.Instance.query(
				"SELECT * FROM game WHERE username = $username",
				{
					username,
				}
			);

			return matches;
		} else {
			const matches = await Surreal.Instance.query("SELECT * FROM game");

			return matches;
		}
	});
}
