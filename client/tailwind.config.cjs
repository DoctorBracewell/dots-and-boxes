const config = {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	mode: "jit",
	darkMode: "media",

	theme: {
		extend: {
			colors: {
				white: "#FFFFFF",
				black: "#050505",
				grey: "#EDEDED",
			},
		},
		fontFamily: {
			body: ["Virgil", "sans-serif"],
		},
	},

	plugins: [],
};

module.exports = config;
