# ainu-utils

[![CI](https://github.com/aynumosir/ainu-utils/actions/workflows/rust.yaml/badge.svg)](https://github.com/aynumosir/ainu-utils/actions/workflows/rust.yaml)
[![codecov](https://codecov.io/gh/aynumosir/ainu-utils/graph/badge.svg?token=aQHfYRVtsd)](https://codecov.io/gh/aynumosir/ainu-utils)

A collection of utility for with the Ainu language

## Releases

ainu-utils is distributed as a Rust crate, but you can also use its binding for Python and Node.js.

| Language |                                                          Version |
| :------- | ---------------------------------------------------------------: |
| Rust     | ![Crates.io Version](https://img.shields.io/crates/v/ainu-utils) |
| Node.js  |          ![npm Version](https://img.shields.io/npm/v/ainu-utils) |
| Python   |        ![PyPI Version](https://img.shields.io/pypi/v/ainu-utils) |

## Features

`ainu-utils` provides several features for working with the Ainu language:

### `tokenize`

Tokenizes Ainu text into morphemes.

```py
from ainu_utils import tokenize

tokenize("irankarapte. e=iwanke ya?", false);
# => ["irankarapte", ".", "e=", "iwanke", "ya?"]
```

### `to_kana`

Converts Ainu text written in Latin script to Kana.

```py
from ainu_utils import to_kana

to_kana("irankarapte. e=iwanke ya?");
# => "イランカラㇷ゚テ。　エイワンケ　ヤ？"
```

### `number_to_words`

Converts integers between 1 and 100 to Ainu words.

```py
from ainu_utils import number_to_words

number_to_words(91);
# => "sine ikasma wan easiknehotne"
```

### `syllabicate`

Parses Ainu text into syllables.

```py
from ainu_utils import syllabicate

syllabicate("irankarapte. e=iwanke ya?")
# => ["i", "ran", "ka", "rap", "te", ".", " ", "e", "=", "i", "wan", "ke", " ", "ya", "?"]
```

## License

MIT
