import ainu_utils


def test_number_to_words():
    result = ainu_utils.number_to_words(21)
    assert result == "sine ikasma hotne"
