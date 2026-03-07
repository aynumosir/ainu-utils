import { test, expect, describe } from "vitest";
import { tokenize, transliterateToKana } from "../dist/index.js";

describe("tokenize", () => {
  test("defaults", () => {
    const tokens = tokenize("irankarapte. e=iwanke ya?");
    expect(tokens).toEqual(["irankarapte", ".", "e=", "iwanke", "ya", "?"]);
  });

  test("keep whitespace", () => {
    const tokens = tokenize("irankarapte. e=iwanke ya?", { keepWhitespace: true });
    expect(tokens).toEqual(["irankarapte", ".", " ", "e=", "iwanke", " ", "ya", "?"]);
  });
});

test("transliterateToKana", () => {
  const tokens = transliterateToKana("irankarapte. e=iwanke ya?");
  expect(tokens).toBe("イランカラㇷ゚テ。　エイワンケ　ヤ？");
});
