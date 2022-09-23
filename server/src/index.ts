import Fastify from "fastify";
import { PORT } from "./constants.js";

const fastify = Fastify({
	logger: {
		transport: {
			target: "pino-pretty",
		},
	},
});

fastify.get("/", async () => {
	return { hello: "world" };
});

try {
	await fastify.listen({ port: PORT });

	console.log(`\n--- Started Server | http://127.0.0.1:${PORT} ---\n`);
} catch (err) {
	fastify.log.error(err);
	process.exit(1);
}
