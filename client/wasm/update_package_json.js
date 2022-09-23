const { writeFile } = require("fs/promises");
const packageJson = require("./pkg/package.json");

packageJson.main = packageJson.module;

writeFile("./pkg/package.json", JSON.stringify(packageJson, null, "\t")).then(
	() => {
		console.log("Successfully updated package.json!");
	}
);
