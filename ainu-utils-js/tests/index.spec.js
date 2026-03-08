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

describe("transliterateToKana", () => {
  test("defaults", () => {
    const tokens = transliterateToKana("irankarapte. e=iwanke ya?");
    expect(tokens).toBe("イランカラㇷ゚テ。　エイワンケ　ヤ？");
  });

  test("ignore pattern", () => {
    const tokens = transliterateToKana("JOHN ku=ne.", {
      ignorePattern: "^[A-Z]+$"
    });
    expect(tokens).toBe("JOHN　クネ。");
  });

  test("throws for an invalid ignore pattern", () => {
    expect(() => {
      transliterateToKana("JOHN ku=ne.", {
        ignorePattern: "[",
      })
    }).toThrowError("Invalid pattern provided");
  });

  test("whitespace", () => {
    const tokens = transliterateToKana("onne paskur ine?", {
      whitespace: "halfwidth",
    });
    expect(tokens).toBe("オンネ パㇱクㇽ イネ？");
  });

  test("throws for an invalid whitespace", () => {
    expect(() => {
      transliterateToKana("irankarapte", {
        whitespace: "xxx",
      })
    }).toThrowError("Invalid whitespace: xxx");
  });
});
