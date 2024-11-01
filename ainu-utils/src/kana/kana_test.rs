use super::kana::to_kana;

#[test]
fn test_to_kana() {
    let sentence = [
        "tanepo Asirpa tak nispa ne kusu, a=kiyannere na.",
        "Asirpa ekimne patek ki wa, menoko monrayke eaykap.",
        "kemeyki ne ya, itese ne ya, menoko monrayke eaykap menoko anak, aynu hoku kor ka eaykap.",
        "tane sinuye kuni paha ne korka, kopan.",
        "Sugimoto nispa, tan matkaci etun wa en=kore!",
        "tan ku=mippoho ku=epotara wa mosir ku=hoppa ka koyaykus.",
    ]
    .join(" ");

    assert_eq!(
		to_kana(&sentence),
		[
            "タネポ　アシㇼパ　タㇰ　ニㇱパ　ネ　クス、　アキヤンネレ　ナ。",
            "アシㇼパ　エキㇺネ　パテㇰ　キ　ワ、　メノコ　モンライケ　エアイカㇷ゚。",
            "ケメイキ　ネ　ヤ、　イテセ　ネ　ヤ、　メノコ　モンライケ　エアイカㇷ゚　メノコ　アナㇰ、　アイヌ　ホク　コㇿ　カ　エアイカㇷ゚。",
            "タネ　シヌイェ　クニ　パハ　ネ　コㇿカ、　コパン。",
            "スギモト　ニㇱパ、　タン　マッカチ　エトゥン　ワ　エンコレ！",
            "タン　クミッポホ　クエポタラ　ワ　モシㇼ　クホッパ　カ　コヤイクㇱ。",
		].join("　")
	)
}

#[test]
fn test_dropping_h() {
    assert_eq!(to_kana("_hine"), "イネ")
}

#[test]
fn test_dropping_y() {
    assert_eq!(to_kana("_ya?"), "ア？")
}

#[test]
fn test_linking_h() {
    assert_eq!(to_kana("hawean __hi"), "ハウェアニ")
}

#[test]
fn test_linking_y() {
    assert_eq!(to_kana("nankor __ya?"), "ナンコラ？")
}

#[test]
fn test_linking_a() {
    assert_eq!(to_kana("cis _a cis _a"), "チサ　チサ")
}

#[test]
fn test_linking_i() {
    assert_eq!(to_kana("oar _isam"), "オアリサㇺ")
}

#[test]
fn test_linking_u() {
    assert_eq!(to_kana("or _un"), "オルン")
}

#[test]
fn test_linking_e() {
    assert_eq!(to_kana("mat _etun"), "マテトゥン")
}

#[test]
fn test_linking_o() {
    assert_eq!(to_kana("pet _or _un"), "ペトルン")
}

#[test]
fn test_linking_and_dropping_a() {
    assert_eq!(to_kana("yaypuri ekira __ani"), "ヤイプリ　エキラニ")
}

#[test]
fn test_linking_and_dropping_i() {
    assert_eq!(to_kana("puni __i"), "プニ")
}

#[test]
fn test_linking_and_dropping_u() {
    assert_eq!(to_kana("a=kotanu __un"), "アコタヌン")
}

#[test]
fn test_linking_and_dropping_e() {
    assert_eq!(to_kana("i=samake __en anu"), "イサマケン　アヌ")
}

#[test]
fn test_linking_and_dropping_o() {
    // 実例なし。
    assert_eq!(to_kana("sapporo __or"), "サッポロㇿ")
}

#[test]
fn test_linking_r_n() {
    assert_eq!(to_kana("a=kor_ nispa"), "アコン　ニㇱパ")
}

#[test]
fn test_linking_r_r() {
    assert_eq!(to_kana("kor_ rusuy"), "コン　ルスイ")
}

#[test]
fn test_linking_r_t() {
    assert_eq!(to_kana("or_ ta"), "オッ　タ")
}

#[test]
fn test_linking_r_c() {
    assert_eq!(to_kana("yar_ cise"), "ヤッ　チセ")
}

#[test]
fn test_linking_n_s() {
    assert_eq!(to_kana("pon_ su"), "ポイ　ス")
}

#[test]
fn test_linking_n_y() {
    assert_eq!(to_kana("pon_ yam"), "ポイ　ヤㇺ")
}

#[test]
fn test_linking_n_w() {
    assert_eq!(to_kana("san _wa"), "サン　マ")
}

#[test]
fn test_linking_m_w() {
    assert_eq!(to_kana("isam _wa"), "イサン　マ")
}

#[test]
fn test_linking_p_w() {
    assert_eq!(to_kana("sap _wa"), "サッ　パ")
}

#[test]
fn test_special_mp() {
    assert_eq!(to_kana("tampaku"), "タンパク")
}

#[test]
fn test_special_mm() {
    assert_eq!(to_kana("umma"), "ウンマ")
}

#[test]
fn test_symbols() {
    assert_eq!(to_kana("“pirka” sekor a=ye"), "「ピㇼカ」　セコㇿ　アイェ")
}

#[test]
fn test_k_prefix() {
    assert_eq!(
        to_kana("irankarapte. kani anak IMO k=e easkay kur ku=ne."),
        "イランカラㇷ゚テ。　カニ　アナㇰ　イモ　ケ　エアㇱカイ　クㇽ　クネ。"
    )
}

#[test]
fn test_diacritics() {
    assert_eq!(to_kana("kamúy"), "カムイ")
}

#[test]
fn test_yy_and_ww() {
    assert_eq!(to_kana("kamuyyukar"), "カムイユカㇻ");
    assert_eq!(to_kana("eawwo"), "エアウウォ");
}

#[test]
fn test_glottal_stop() {
    assert_eq!(to_kana("hioy'oy"), "ヒオイオイ");
}
