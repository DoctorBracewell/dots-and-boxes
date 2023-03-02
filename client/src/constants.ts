const DEVELOPMENT = import.meta.env.MODE === "development";

/*
 * Default colours
 */
export const RED = "#F50400";
export const BLUE = "#2892D7";

/*
 * Leaderboard API URL data
 */
export const URL = DEVELOPMENT ? "http://127.0.0.1" : "todo";
export const PORT = DEVELOPMENT ? 3000 : 3000;
