import { writeFile } from "fs/promises";
import packageJson from "./pkg/package.json" assert { type: "json" };

packageJson.main = packageJson.module;

writeFile("./pkg/package.json", JSON.stringify(packageJson, null, "\t")).then(
	() => {
		console.log("Successfully updated package.json!");
	}
);
