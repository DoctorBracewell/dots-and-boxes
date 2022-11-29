const config = {
	content: ["./src/**/*.{html,js,svelte,ts}"],

	theme: {
		extend: {
			colors: {
				white: "#FCFCFC",
				black: "#050505",
				red: "#F50400",
				blue: "#2892D7",
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
