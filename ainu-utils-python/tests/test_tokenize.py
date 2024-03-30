import ainu_utils


def test_tokenize():
    result = ainu_utils.segment("irankarapte. e=iwanke ya?")
    assert result == ["irankarapte", ".", "e=", "iwanke", "ya", "?"]
