import ainu_utils


def test_to_kana():
    result = ainu_utils.to_kana("irankarapte. e=iwanke ya?")
    assert result == "イランカラㇷ゚テ。　エイワンケ　ヤ？"
