import ainu_utils

def test_number_to_words():
    result = ainu_utils.number_to_words(21, "adnominal", noun="yuk")
    assert result == "sine yuk ikasma hotne yuk"
