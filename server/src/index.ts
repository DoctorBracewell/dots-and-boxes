import cors from "@fastify/cors";
import * as dotenv from "dotenv";
import Fastify from "fastify";
import { readdir } from "fs/promises";

import { PORT, RATE_LIMITS } from "./constants.js";
import surrealInit from "./database.js";

// Load environment variables and connect to database
await dotenv.config();
await surrealInit();

// Create Fastify instance
const fastify = Fastify({
	logger: {
		transport: {
			target: "pino-pretty",
		},
	},
});

// Disable CORS
await fastify.register(cors, {
	origin: "*",
});

// Rate limit requests to 100 per minute
await fastify.register(import("@fastify/rate-limit"), RATE_LIMITS);

// Initialise all routes
try {
	const routes = await readdir("./build/routes");

	for (const route of routes) {
		const { setupRoutes } = await import(`./routes/${route}`);

		setupRoutes(fastify);
	}
} catch (err) {
	console.error(err);
	process.exit(1);
}

// Start server
try {
	await fastify.listen({ port: PORT });

	console.log(`\n--- Started Server | http://127.0.0.1:${PORT} ---\n`);
} catch (err) {
	fastify.log.error(err);
	process.exit(1);
}
