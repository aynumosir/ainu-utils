# ainu-utils

[![CI: Rust](https://github.com/aynumosir/ainu-utils/actions/workflows/ci_rust.yaml/badge.svg)](https://github.com/aynumosir/ainu-utils/actions/workflows/ci_rust.yaml)
[![codecov](https://codecov.io/gh/aynumosir/ainu-utils/graph/badge.svg?token=aQHfYRVtsd)](https://codecov.io/gh/aynumosir/ainu-utils)

A toolkit for Ainu language processing, available in Rust, JavaScript, and Python.

## Releases

ainu-utils is distributed as a Rust crate, but you can also use its binding for Python and Node.js.

| Language |                                                          Version |
| :------- | ---------------------------------------------------------------: |
| Rust     | ![Crates.io Version](https://img.shields.io/crates/v/ainu-utils) |
| Node.js  |          ![npm Version](https://img.shields.io/npm/v/ainu-utils) |
| Python   |        ![PyPI Version](https://img.shields.io/pypi/v/ainu-utils) |

## Features

`ainu-utils` provides several features for working with the Ainu language:

### Tokenization

Tokenizes Ainu text into morphemes.

**Python:**

```py
from ainu_utils import tokenize

tokenize("irankarapte. e=iwanke ya?", keep_whitespace=False)
# => ["irankarapte", ".", "e=", "iwanke", "ya?"]
```

**JS:**

```js
import { tokenize } from "ainu-utils";

tokenize("irankarapte. e=iwanke ya?", { keepWhitespace: false });
// => ["irankarapte", ".", "e=", "iwanke", "ya?"]
```

### Syllabication

Parses Ainu text into syllables.

**Python:**

```py
from ainu_utils import syllabicate

syllabicate("irankarapte. e=iwanke ya?")
# => ["i", "ran", "ka", "rap", "te", ".", " ", "e", "=", "i", "wan", "ke", " ", "ya", "?"]
```

**JS:**

```js
import { syllabicate } from "ainu-utils";

syllabicate("irankarapte. e=iwanke ya?")
// => ["i", "ran", "ka", "rap", "te", ".", " ", "e", "=", "i", "wan", "ke", " ", "ya", "?"]
```


### Transliteration

Converts Ainu text written in Latin script to Kana.

**Python:**

```py
from ainu_utils import transliterate_to_kana, Whitespace

transliterate_to_kana(
    "irankarapte. e=iwanke ya?",
    whitespace=Whitespace.Fullwidth,
    ignore_pattern=None,
)
# => "イランカラㇷ゚テ。　エイワンケ　ヤ？"
```

**JS:**

```js
import { transliterateToKana } from "ainu-utils";

transliterateToKana("irankarapte. e=iwanke ya?", {
  whitespace: "fullwidth",
  ignore_pattern: null,
});
// => "イランカラㇷ゚テ。　エイワンケ　ヤ？"
```

### Convertion from a number to words

Converts integers between 1 and 100 to Ainu words.

**Python:**

```py
from ainu_utils import number_to_words

number_to_words(91)
# => "sine ikasma wan easiknehotne"
```

**JS:**

```js
import { numberToWords } from "ainu-utils";

numberToWords(91);
// => "sine ikasma wan easiknehotne"
```

## License

MIT
