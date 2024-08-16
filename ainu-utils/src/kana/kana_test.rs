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
			"タネポ アシㇼパ タㇰ ニㇱパ ネ クス、 アキヤンネレ ナ。",
			"アシㇼパ エキㇺネ パテㇰ キ ワ、 メノコ モンライケ エアイカㇷ゚。",
			"ケメイキ ネ ヤ、 イテセ ネ ヤ、 メノコ モンライケ エアイカㇷ゚ メノコ アナㇰ、 アイヌ ホク コㇿ カ エアイカㇷ゚。",
			"タネ シヌイェ クニ パハ ネ コㇿカ、 コパン。",
			"スgイモト ニㇱパ、 タン マッカチ エトゥン ワ エンコレ!",
			"タン クミッポホ クエポタラ ワ モシㇼ クホッパ カ コヤイクㇱ。",
		].join(" ")
	)
}
