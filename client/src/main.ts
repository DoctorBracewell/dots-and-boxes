/**
 * @author DrBracewell
 * @copyright 2023
 */
import App from "./App.svelte";

import "./app.css";

/*
 * Initialise application
 */
const app = new App({
	target: document.getElementById("app"),
});

export default app;
