export const DEV = process.env.NODE_ENV === "development";
export const PORT = 3000;

export const RATE_LIMITS = {
	max: 30,
	timeWindow: 1000 * 60,
};
