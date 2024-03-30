import test from "ava";

import { segment } from "../index.js";

test("segment", (t) => {
  t.deepEqual(segment("irankarapte. e=iwanke ya?"), [
    "irankarapte",
    ".",
    "e=",
    "iwanke",
    "ya",
    "?",
  ]);
});
