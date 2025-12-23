import { readFile, writeFile } from "fs/promises";
const packageJson = JSON.parse(await readFile(new URL('./pkg/package.json', import.meta.url), 'utf-8'));

packageJson.main = packageJson.module;

writeFile("./pkg/package.json", JSON.stringify(packageJson, null, "\t")).then(
	() => {
		console.log("Successfully updated package.json!");
	}
);
