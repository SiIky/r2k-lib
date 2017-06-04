// TODO: Complete this list
#![allow(dead_code)]
//  - [X] Punctuation
pub const PUNCT_PERIOD: &str = "\u{3002}"; // 。
pub const PUNCT_COMMA: &str = "\u{3001}"; // 、
pub const PUNCT_EMARK: &str = "\u{ff01}"; // ！
pub const PUNCT_QMARK: &str = "\u{ff1f}"; // ？
pub const PUNCT_CHOONPU: &str = "\u{30fc}"; // ー

//  - [X] Hiragana
//      - [X] Without diacritics
//      - [X] With diacritics
pub const HIRA_A: &str = "\u{3042}"; // あ
pub const HIRA_SMALL_A: &str = "\u{3042}"; // ぁ
pub const HIRA_I: &str = "\u{3044}"; // い
pub const HIRA_SMALL_I: &str = "\u{3044}"; // ぃ
pub const HIRA_U: &str = "\u{3046}"; // う
pub const HIRA_SMALL_U: &str = "\u{3046}"; // ぅ
pub const HIRA_E: &str = "\u{3048}"; // え
pub const HIRA_SMALL_E: &str = "\u{3048}"; // ぇ
pub const HIRA_O: &str = "\u{304a}"; // お
pub const HIRA_SMALL_O: &str = "\u{304a}"; // ぉ
pub const HIRA_KA: &str = "\u{304b}"; // か
pub const HIRA_GA: &str = "\u{304c}"; // が
pub const HIRA_KI: &str = "\u{304d}"; // き
pub const HIRA_GI: &str = "\u{304e}"; // ぎ
pub const HIRA_KU: &str = "\u{304f}"; // く
pub const HIRA_GU: &str = "\u{3050}"; // ぐ
pub const HIRA_KE: &str = "\u{3051}"; // け
pub const HIRA_GE: &str = "\u{3052}"; // げ
pub const HIRA_KO: &str = "\u{3053}"; // こ
pub const HIRA_GO: &str = "\u{3054}"; // ご
pub const HIRA_SA: &str = "\u{3055}"; // さ
pub const HIRA_ZA: &str = "\u{3056}"; // ざ
pub const HIRA_SHI: &str = "\u{3057}"; // し
pub const HIRA_JI: &str = "\u{3058}"; // じ
pub const HIRA_SU: &str = "\u{3059}"; // す
pub const HIRA_ZU: &str = "\u{305a}"; // ず
pub const HIRA_SE: &str = "\u{305b}"; // せ
pub const HIRA_ZE: &str = "\u{305c}"; // ぜ
pub const HIRA_SO: &str = "\u{305d}"; // そ
pub const HIRA_ZO: &str = "\u{305e}"; // ぞ
pub const HIRA_TA: &str = "\u{305f}"; // た
pub const HIRA_DA: &str = "\u{3060}"; // だ
pub const HIRA_CHI: &str = "\u{3061}"; // ち
pub const HIRA_DJI: &str = "\u{3062}"; // ぢ
pub const HIRA_SMALL_TSU: &str = "\u{3063}"; // っ
pub const HIRA_TSU: &str = "\u{3064}"; // つ
pub const HIRA_DZU: &str = "\u{3065}"; // づ
pub const HIRA_TE: &str = "\u{3066}"; // て
pub const HIRA_DE: &str = "\u{3067}"; // で
pub const HIRA_TO: &str = "\u{3068}"; // と
pub const HIRA_DO: &str = "\u{3069}"; // ど
pub const HIRA_NA: &str = "\u{306a}"; // な
pub const HIRA_NI: &str = "\u{306b}"; // に
pub const HIRA_NU: &str = "\u{306c}"; // ぬ
pub const HIRA_NE: &str = "\u{306d}"; // ね
pub const HIRA_NO: &str = "\u{306d}"; // の
pub const HIRA_HA: &str = "\u{306f}"; // は
pub const HIRA_BA: &str = "\u{3070}"; // ば
pub const HIRA_PA: &str = "\u{3071}"; // ぱ
pub const HIRA_HI: &str = "\u{3072}"; // ひ
pub const HIRA_BI: &str = "\u{3073}"; // び
pub const HIRA_PI: &str = "\u{3074}"; // ぴ
pub const HIRA_FU: &str = "\u{3075}"; // ふ
pub const HIRA_BU: &str = "\u{3076}"; // ぶ
pub const HIRA_PU: &str = "\u{3077}"; // ぷ
pub const HIRA_HE: &str = "\u{3078}"; // へ
pub const HIRA_BE: &str = "\u{3079}"; // べ
pub const HIRA_PE: &str = "\u{307a}"; // ぺ
pub const HIRA_HO: &str = "\u{307b}"; // ほ
pub const HIRA_BO: &str = "\u{307c}"; // ぼ
pub const HIRA_PO: &str = "\u{307d}"; // ぽ
pub const HIRA_MA: &str = "\u{307e}"; // ま
pub const HIRA_MI: &str = "\u{307f}"; // み
pub const HIRA_MU: &str = "\u{3080}"; // む
pub const HIRA_ME: &str = "\u{3081}"; // め
pub const HIRA_MO: &str = "\u{3082}"; // も
pub const HIRA_SMALL_YA: &str = "\u{3083}"; // ゃ
pub const HIRA_YA: &str = "\u{3084}"; // や
pub const HIRA_SMALL_YO: &str = "\u{3087}"; // ょ
pub const HIRA_YO: &str = "\u{3088}"; // よ
pub const HIRA_SMALL_YU: &str = "\u{3085}"; // ゅ
pub const HIRA_YU: &str = "\u{3086}"; // ゆ
pub const HIRA_RA: &str = "\u{3089}"; // ら
pub const HIRA_RI: &str = "\u{308a}"; // り
pub const HIRA_RU: &str = "\u{308b}"; // る
pub const HIRA_RE: &str = "\u{308c}"; // れ
pub const HIRA_RO: &str = "\u{308d}"; // ろ
pub const HIRA_WA: &str = "\u{308f}"; // わ
pub const HIRA_WI: &str = "\u{3090}"; // ゐ
pub const HIRA_WE: &str = "\u{3091}"; // ゑ
pub const HIRA_WO: &str = "\u{3092}"; // を
pub const HIRA_N: &str = "\u{3093}"; // ん
pub const HIRA_VU: &str = "\u{3094}"; // ゔ

//  - [X] Katakana
//      - [X] Without diacritics
//      - [X] With diacritics
pub const KATA_A: &str = "\u{30a2}"; // ア
pub const KATA_SMALL_A: &str = "\u{30a1}"; // ァ
pub const KATA_I: &str = "\u{30a4}"; // イ
pub const KATA_SMALL_I: &str = "\u{30a3}"; // ィ
pub const KATA_U: &str = "\u{30a6}"; // ウ
pub const KATA_SMALL_U: &str = "\u{30a5}"; // ゥ
pub const KATA_E: &str = "\u{30a8}"; // エ
pub const KATA_SMALL_E: &str = "\u{30a7}"; // ェ
pub const KATA_O: &str = "\u{30aa}"; // オ
pub const KATA_SMALL_O: &str = "\u{30a9}"; // ォ
pub const KATA_KA: &str = "\u{30ab}"; // カ
pub const KATA_GA: &str = "\u{30ac}"; // ガ
pub const KATA_KI: &str = "\u{30ad}"; // キ
pub const KATA_GI: &str = "\u{30ae}"; // ギ
pub const KATA_KU: &str = "\u{30af}"; // ク
pub const KATA_GU: &str = "\u{30b0}"; // グ
pub const KATA_KE: &str = "\u{30b1}"; // ケ
pub const KATA_GE: &str = "\u{30b2}"; // ゲ
pub const KATA_KO: &str = "\u{30b3}"; // コ
pub const KATA_GO: &str = "\u{30b4}"; // ゴ
pub const KATA_SA: &str = "\u{30b5}"; // サ
pub const KATA_ZA: &str = "\u{30b6}"; // ザ
pub const KATA_SHI: &str = "\u{30b7}"; // シ
pub const KATA_JI: &str = "\u{30b8}"; // ジ
pub const KATA_SU: &str = "\u{30b9}"; // ス
pub const KATA_ZU: &str = "\u{30ba}"; // ズ
pub const KATA_SE: &str = "\u{30bb}"; // セ
pub const KATA_ZE: &str = "\u{30bc}"; // ゼ
pub const KATA_SO: &str = "\u{30bd}"; // ソ
pub const KATA_ZO: &str = "\u{30be}"; // ゾ
pub const KATA_TA: &str = "\u{30bf}"; // タ
pub const KATA_DA: &str = "\u{30c0}"; // ダ
pub const KATA_CHI: &str = "\u{30c1}"; // チ
pub const KATA_DJI: &str = "\u{30c2}"; // ヂ
pub const KATA_SMALL_TSU: &str = "\u{30c3}"; // ッ
pub const KATA_TSU: &str = "\u{30c4}"; // ツ
pub const KATA_DZU: &str = "\u{30c5}"; // ヅ
pub const KATA_TE: &str = "\u{30c6}"; // テ
pub const KATA_DE: &str = "\u{30c7}"; // デ
pub const KATA_TO: &str = "\u{30c8}"; // ト
pub const KATA_DO: &str = "\u{30c9}"; // ド
pub const KATA_NA: &str = "\u{30ca}"; // ナ
pub const KATA_NI: &str = "\u{30cb}"; // ニ
pub const KATA_NU: &str = "\u{30cc}"; // ヌ
pub const KATA_NE: &str = "\u{30cd}"; // ネ
pub const KATA_NO: &str = "\u{30ce}"; // ノ
pub const KATA_HA: &str = "\u{30cf}"; // ハ
pub const KATA_BA: &str = "\u{30d0}"; // バ
pub const KATA_PA: &str = "\u{30d1}"; // パ
pub const KATA_HI: &str = "\u{30d2}"; // ヒ
pub const KATA_BI: &str = "\u{30d3}"; // ビ
pub const KATA_PI: &str = "\u{30d4}"; // ピ
pub const KATA_FU: &str = "\u{30d5}"; // フ
pub const KATA_BU: &str = "\u{30d6}"; // ブ
pub const KATA_PU: &str = "\u{30d7}"; // プ
pub const KATA_HE: &str = "\u{30d8}"; // ヘ
pub const KATA_BE: &str = "\u{30d9}"; // ベ
pub const KATA_PE: &str = "\u{30da}"; // ペ
pub const KATA_HO: &str = "\u{30db}"; // ホ
pub const KATA_BO: &str = "\u{30dc}"; // ボ
pub const KATA_PO: &str = "\u{30dd}"; // ポ
pub const KATA_MA: &str = "\u{30de}"; // マ
pub const KATA_MI: &str = "\u{30df}"; // ミ
pub const KATA_MU: &str = "\u{30e0}"; // ム
pub const KATA_ME: &str = "\u{30e1}"; // メ
pub const KATA_MO: &str = "\u{30e2}"; // モ
pub const KATA_SMALL_YA: &str = "\u{30e3}"; // ャ
pub const KATA_YA: &str = "\u{30e4}"; // ヤ
pub const KATA_SMALL_YU: &str = "\u{30e5}"; // ュ
pub const KATA_YU: &str = "\u{30e6}"; // ユ
pub const KATA_SMALL_YO: &str = "\u{30e7}"; // ョ
pub const KATA_YO: &str = "\u{30e8}"; // ヨ
pub const KATA_RA: &str = "\u{30e9}"; // ラ
pub const KATA_RI: &str = "\u{30ea}"; // リ
pub const KATA_RU: &str = "\u{30eb}"; // ル
pub const KATA_RE: &str = "\u{30ec}"; // レ
pub const KATA_RO: &str = "\u{30ed}"; // ロ
pub const KATA_WA: &str = "\u{30ef}"; // ワ
pub const KATA_WI: &str = "\u{30f0}"; // ヰ
pub const KATA_WE: &str = "\u{30f1}"; // ヱ
pub const KATA_WO: &str = "\u{30f2}"; // ヲ
pub const KATA_N: &str = "\u{30f3}"; // ン
pub const KATA_VU: &str = "\u{30f4}"; // ヴ
