import Surreal from "surrealdb.js";

export default async function () {
	await Surreal.Instance.connect("http://127.0.0.1:8000/rpc");

	await Surreal.Instance.signin({
		user: process.env.SURREAL_USER ?? "",
		pass: process.env.SURREAL_PASS ?? "",
	});

	await Surreal.Instance.use("dots-and-boxes", "dots-and-boxes");
}
