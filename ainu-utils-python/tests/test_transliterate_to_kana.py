import ainu_utils


def test_transliterate_to_kana():
    result = ainu_utils.transliterate_to_kana("irankarapte. e=iwanke ya?")
    assert result == "イランカラㇷ゚テ。　エイワンケ　ヤ？"


def test_transliterate_to_kana_halfwidth():
    result = ainu_utils.transliterate_to_kana(
        "irankarapte. e=iwanke ya?", whitespace=ainu_utils.Whitespace.Halfwidth
    )
    assert result == "イランカラㇷ゚テ。 エイワンケ ヤ？"


def test_transliterate_to_kana_ignore():
    result = ainu_utils.transliterate_to_kana("JOHN ku=ne.", ignore_pattern="^[A-Z]+$")
    assert result == "JOHN　クネ。"
