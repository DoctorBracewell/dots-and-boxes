import Fastify from "fastify";
import { PORT } from "./constants.js";
import surrealInit from "./database.js";
import * as dotenv from "dotenv";
import { readdir } from "fs/promises";
import { dirname } from "path";

await dotenv.config();
await surrealInit();

const fastify = Fastify({
	logger: {
		transport: {
			target: "pino-pretty",
		},
	},
});

await fastify.register(import("@fastify/rate-limit"), {
	max: 100,
	timeWindow: 1000 * 60,
});

try {
	const routes = await readdir("./build/routes");

	for (const route of routes) {
		const { setupRoutes } = await import(`./routes/${route}`);

		console.log(setupRoutes);

		setupRoutes(fastify);
	}
} catch (err) {
	console.error(err);
	process.exit(1);
}

try {
	await fastify.listen({ port: PORT });

	console.log(`\n--- Started Server | http://127.0.0.1:${PORT} ---\n`);
} catch (err) {
	fastify.log.error(err);
	process.exit(1);
}
