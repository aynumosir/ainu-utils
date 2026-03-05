use crate::kana::KanaConfig;

use super::kana::transliterate;

fn transliterate_default(input: &str) -> String {
    transliterate(input, &KanaConfig::default())
}

#[test]
fn test_transliterate_default() {
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
        transliterate_default(&sentence),
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
    assert_eq!(transliterate_default("_hine"), "イネ")
}

#[test]
fn test_dropping_y() {
    assert_eq!(transliterate_default("_ya?"), "ア？")
}

#[test]
fn test_linking_h() {
    assert_eq!(transliterate_default("hawean __hi"), "ハウェアニ")
}

#[test]
fn test_linking_y() {
    assert_eq!(transliterate_default("nankor __ya?"), "ナンコラ？")
}

#[test]
fn test_linking_a() {
    assert_eq!(transliterate_default("cis _a cis _a"), "チサ　チサ")
}

#[test]
fn test_linking_i() {
    assert_eq!(transliterate_default("oar _isam"), "オアリサㇺ")
}

#[test]
fn test_linking_u() {
    assert_eq!(transliterate_default("or _un"), "オルン")
}

#[test]
fn test_linking_e() {
    assert_eq!(transliterate_default("mat _etun"), "マテトゥン")
}

#[test]
fn test_linking_o() {
    assert_eq!(transliterate_default("pet _or _un"), "ペトルン")
}

#[test]
fn test_linking_and_dropping_a() {
    assert_eq!(
        transliterate_default("yaypuri ekira __ani"),
        "ヤイプリ　エキラニ"
    )
}

#[test]
fn test_linking_and_dropping_i() {
    assert_eq!(transliterate_default("puni __i"), "プニ")
}

#[test]
fn test_linking_and_dropping_u() {
    assert_eq!(transliterate_default("a=kotanu __un"), "アコタヌン")
}

#[test]
fn test_linking_and_dropping_e() {
    assert_eq!(
        transliterate_default("i=samake __en anu"),
        "イサマケン　アヌ"
    )
}

#[test]
fn test_linking_and_dropping_o() {
    // 実例なし。
    assert_eq!(transliterate_default("sapporo __or"), "サッポロㇿ")
}

#[test]
fn test_linking_r_n() {
    assert_eq!(transliterate_default("a=kor_ nispa"), "アコン　ニㇱパ")
}

#[test]
fn test_linking_r_r() {
    assert_eq!(transliterate_default("kor_ rusuy"), "コン　ルスイ")
}

#[test]
fn test_linking_r_t() {
    assert_eq!(transliterate_default("or_ ta"), "オッ　タ")
}

#[test]
fn test_linking_r_c() {
    assert_eq!(transliterate_default("yar_ cise"), "ヤッ　チセ")
}

#[test]
fn test_linking_n_s() {
    assert_eq!(transliterate_default("pon_ su"), "ポイ　ス")
}

#[test]
fn test_linking_n_y() {
    assert_eq!(transliterate_default("pon_ yam"), "ポイ　ヤㇺ")
}

#[test]
fn test_linking_n_w() {
    assert_eq!(transliterate_default("san _wa"), "サン　マ")
}

#[test]
fn test_linking_m_w() {
    assert_eq!(transliterate_default("isam _wa"), "イサン　マ")
}

#[test]
fn test_linking_p_w() {
    assert_eq!(transliterate_default("sap _wa"), "サッ　パ")
}

#[test]
fn test_special_mp() {
    assert_eq!(transliterate_default("tampaku"), "タンパク")
}

#[test]
fn test_special_mm() {
    assert_eq!(transliterate_default("umma"), "ウンマ")
}

#[test]
fn test_symbols() {
    assert_eq!(
        transliterate_default("“pirka” sekor a=ye"),
        "「ピㇼカ」　セコㇿ　アイェ"
    )
}

#[test]
fn test_k_prefix() {
    assert_eq!(transliterate_default("k=e"), "ケ")
}

#[test]
fn test_diacritics() {
    assert_eq!(transliterate_default("kamúy"), "カムイ")
}

#[test]
fn test_yy_and_ww() {
    assert_eq!(transliterate_default("kamuyyukar"), "カムイユカㇻ");
    assert_eq!(transliterate_default("eawwo"), "エアウウォ");
}

#[test]
fn test_glottal_stop() {
    assert_eq!(transliterate_default("hioy'oy"), "ヒオイオイ");
}
<<<<<<< Updated upstream
=======

#[test]
fn test_rollback() {
    assert_eq!(
        transliterate_default("Copyright Mojang AB. iteki eymek yan!"),
        "Copyright　Mojang　AB.　イテキ　エイメㇰ　ヤン！"
    )
}

#[test]
fn test_ignore_words() {
    let config = KanaConfig {
        ignore_words: vec!["aynu.io".to_string()],
        ..KanaConfig::default()
    };

    assert_eq!(
        transliterate("aynu.io nukar yan", &config),
        "aynu.io　ヌカㇻ　ヤン"
    )
}
>>>>>>> Stashed changes
