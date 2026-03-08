use super::kana::{IgnorePattern, TransliterateToKanaOptions, Whitespace, transliterate_to_kana};

#[test]
fn test_transliterate_to_kana() {
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
        transliterate_to_kana(&sentence, &Default::default()),
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
fn test_linking() {
    assert_eq!(transliterate_to_kana("_hine", &Default::default()), "イネ");
    assert_eq!(transliterate_to_kana("_ya?", &Default::default()), "ア？");
    assert_eq!(
        transliterate_to_kana("hawean __hi", &Default::default()),
        "ハウェアニ"
    );
    assert_eq!(
        transliterate_to_kana("nankor __ya?", &Default::default()),
        "ナンコラ？"
    );
    assert_eq!(
        transliterate_to_kana("cis _a cis _a", &Default::default()),
        "チサ　チサ"
    );
    assert_eq!(
        transliterate_to_kana("oar _isam", &Default::default()),
        "オアリサㇺ"
    );
    assert_eq!(
        transliterate_to_kana("or _un", &Default::default()),
        "オルン"
    );
    assert_eq!(
        transliterate_to_kana("mat _etun", &Default::default()),
        "マテトゥン"
    );
    assert_eq!(
        transliterate_to_kana("pet _or _un", &Default::default()),
        "ペトルン"
    );
    assert_eq!(
        transliterate_to_kana("yaypuri ekira __ani", &Default::default()),
        "ヤイプリ　エキラニ"
    );
    assert_eq!(
        transliterate_to_kana("puni __i", &Default::default()),
        "プニ"
    );
    assert_eq!(
        transliterate_to_kana("a=kotanu __un", &Default::default()),
        "アコタヌン"
    );
    assert_eq!(
        transliterate_to_kana("i=samake __en anu", &Default::default()),
        "イサマケン　アヌ"
    );
    // 実例なし。
    assert_eq!(
        transliterate_to_kana("sapporo __or", &Default::default()),
        "サッポロㇿ"
    );
    assert_eq!(
        transliterate_to_kana("a=kor_ nispa", &Default::default()),
        "アコン　ニㇱパ"
    );
    assert_eq!(
        transliterate_to_kana("kor_ rusuy", &Default::default()),
        "コン　ルスイ"
    );
    assert_eq!(
        transliterate_to_kana("or_ ta", &Default::default()),
        "オッ　タ"
    );
    assert_eq!(
        transliterate_to_kana("yar_ cise", &Default::default()),
        "ヤッ　チセ"
    );
    assert_eq!(
        transliterate_to_kana("pon_ su", &Default::default()),
        "ポイ　ス"
    );
    assert_eq!(
        transliterate_to_kana("pon_ yam", &Default::default()),
        "ポイ　ヤㇺ"
    );
    assert_eq!(
        transliterate_to_kana("san _wa", &Default::default()),
        "サン　マ"
    );
    assert_eq!(
        transliterate_to_kana("isam _wa", &Default::default()),
        "イサン　マ"
    );
    assert_eq!(
        transliterate_to_kana("sap _wa", &Default::default()),
        "サッ　パ"
    );
}

#[test]
fn test_special_consonant_clusters() {
    assert_eq!(
        transliterate_to_kana("tampaku", &Default::default()),
        "タンパク"
    );
    assert_eq!(transliterate_to_kana("umma", &Default::default()), "ウンマ");
    assert_eq!(
        transliterate_to_kana("kamuyyukar", &Default::default()),
        "カムイユカㇻ"
    );
    assert_eq!(
        transliterate_to_kana("eawwo", &Default::default()),
        "エアウウォ"
    );
}

#[test]
fn test_symbols() {
    assert_eq!(
        transliterate_to_kana("“pirka” sekor a=ye", &Default::default()),
        "「ピㇼカ」　セコㇿ　アイェ"
    )
}

#[test]
fn test_sakhalin_ainu() {
    assert_eq!(
        transliterate_to_kana("ah ih uh eh oh", &Default::default()),
        "アㇵ　イㇶ　ウㇷ　エㇸ　オㇹ"
    )
}

#[test]
fn test_k_prefix() {
    assert_eq!(
        transliterate_to_kana("ku=ne ruwe ne", &Default::default()),
        "クネ　ルウェ　ネ"
    )
}

#[test]
fn test_diacritics() {
    assert_eq!(
        transliterate_to_kana("kamúy", &Default::default()),
        "カムイ"
    );
    assert_eq!(
        transliterate_to_kana("hioy'oy", &Default::default()),
        "ヒオイオイ"
    );
}

#[test]
fn test_halfwidth() {
    let options = TransliterateToKanaOptions {
        whitespace: Whitespace::Halfwidth,
        ..Default::default()
    };
    assert_eq!(
        transliterate_to_kana("ku=iki kusu ne na", &options),
        "クイキ クス ネ ナ"
    )
}

#[test]
fn test_ignore_pattern() {
    let options = TransliterateToKanaOptions {
        ignore_pattern: Some(IgnorePattern::new("^[A-Z]+$").unwrap()),
        ..Default::default()
    };

    assert_eq!(
        transliterate_to_kana("JOHN ku=ne wa.", &options),
        "JOHN　クネ　ワ。"
    )
}
