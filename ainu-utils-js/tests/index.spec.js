import { test, expect } from "vitest";
import { tokenize, transliterateToKana } from "../dist/index.js";

test("tokenize", () => {
  const tokens = tokenize("irankarapte. e=iwanke ya?", { keepWhitespace: false });
  expect(tokens).toEqual(["irankarapte", ".", "e=", "iwanke", "ya", "?"]);
});


test("transliterateToKana", () => {
  const tokens = transliterateToKana("irankarapte. e=iwanke ya?");
  expect(tokens).toBe("イランカラㇷ゚テ。　エイワンケ　ヤ？");
});
