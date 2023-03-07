import type { FastifyInstance } from "fastify";
import { basename } from "path";
import Surreal from "surrealdb.js";

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

interface Match extends Body {
	game_time: string;
	id: string;
}

const route = basename(import.meta.url, ".js");

// Validate a username
export function checkUsername(username: string) {
	const checks = [
		{ check: username.length >= 1, string: "Username too short" },
		{ check: username.length <= 20, string: "Username too long" },
		{
			check: /^[a-zA-Z0-9_]+$/.test(username),
			string: "Username uses invalid characters",
		},
	];

	return checks.filter((check) => !check.check);
}

export function setupRoutes(server: FastifyInstance) {
	server.post<{
		Body: Body;
	}>(`/${route}`, async (request, response) => {
		const { username, result, board } = request.body;
		const failedChecks = checkUsername(username);

		if (failedChecks.length > 0) {
			return response
				.status(400)
				.send(`${failedChecks.map((c) => c.string).join(".\n")}`);
		}

		if (!checkUsername(username))
			return response.status(400).send("Invalid username length");

		await Surreal.Instance.query(
			"CREATE game SET username = $username, result = $result, game_time = time::now(), board = $board",
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

			const matches: Match[] =
				(await Surreal.Instance.query(
					"SELECT * FROM game WHERE username = $username",
					{
						username,
					}
				)) || [];

			return matches;
		} else {
			const matches: Match[] = await Surreal.Instance.query(
				"SELECT * FROM game"
			);

			return matches;
		}
	});
}
