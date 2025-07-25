import ainu_utils


def test_syllabicate():
    result = ainu_utils.syllabicate("irankarapte. e=iwanke ya?")
    assert result == [
        "i",
        "ran",
        "ka",
        "rap",
        "te",
        ".",
        " ",
        "e",
        "=",
        "i",
        "wan",
        "ke",
        " ",
        "ya",
        "?",
    ]
