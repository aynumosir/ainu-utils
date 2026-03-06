import { test, expect } from "vitest";
import { numbers_to_words } from "../pkg/ainu_utils_js.js";

test("numbers_to_words", () => {
	const words = numbers_to_words(31, "adnominal", "cape");
	expect(words).toBe("sine cape ikasma wan cape etuhotne cape");
});
