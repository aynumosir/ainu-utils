import ainu_utils


def test_tokenize():
    result = ainu_utils.segment("irankarapte. e=iwanke ya?", keep_whitespace=False)
    assert result == ["irankarapte", ".", "e=", "iwanke", "ya", "?"]


def test_tokenize_with_whitespace():
    result = ainu_utils.segment("irankarapte. e=iwanke ya?", keep_whitespace=True)
    assert result == ["irankarapte", ".", " ", "e=", "iwanke", " ", "ya", "?"]
