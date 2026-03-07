import { test, expect } from "vitest";
import { tokenize } from "../dist/index.js";

test("tokenize", () => {
  const tokens = tokenize("irankarapte. e=iwanke ya?", false);
  expect(tokens).toEqual(["irankarapte", ".", "e=", "iwanke", "ya", "?"]);
});
