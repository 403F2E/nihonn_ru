/// Constants used in the commands and help sentences
use phf::{phf_map, Map};
/// Romanji sounding for the readings
pub const HIRAGANA_ROMANJI: Map<char, &'static str> = phf_map!(
    'あ' =>	"a",	'い' => "i", 'う' => "u",	'え' => "e", 'お' => "o",
    'か' => "ka", 'き' => "ki", 'く' => "ku", 'け' => "ke", 'こ' => "ko",
    'さ' => "sa", 'し' => "shi", 'す' => "su", 'せ' => "se", 'そ' => "so",
    'た' => "ta", 'ち' => "chi", 'つ' => "tsu", 'て' => "te", 'と' => "to",
    'な' => "na", 'に' => "ni", 'ぬ' => "nu", 'ね' => "ne", 'の' => "no",
    'は' => "ha", 'ひ' => "hi", 'ふ' => "fu", 'へ' => "he", 'ほ' => "ho",
    'ま' => "ma", 'み' => "mi", 'む' => "mu", 'め' => "me", 'も' => "mo",
    'や' => "ya", 'ゆ' => "yu",	'よ' => "yo",
    'ら' => "ra", 'り' => "ri", 'る' => "ru", 'れ' => "re", 'ろ' => "ro",
    'わ' => "wa", 'ゐ' => "wi",	'ゑ' => "we", 'を' => "wo/o",
    'ん' => "n/m",
    'が' => "ga", 'ぎ' => "gi", 'ぐ' => "gu", 'げ' => "ge", 'ご' => "go",
    'ざ' => "za", 'じ' => "ji", 'ず' => "zu", 'ぜ' => "ze", 'ぞ' => "zo",
    'だ' => "da", 'ぢ' => "di", 'づ' => "du",	'で' => "de", 'ど' => "do",
    'ば' => "ba", 'び' => "bi", 'ぶ' => "bu", 'べ' => "be", 'ぼ' => "bo",
    'ぱ' => "pa", 'ぴ' => "pi", 'ぷ' => "pu", 'ぺ' => "pe", 'ぽ' => "po",
    ' ' => " ", 'ゃ' => "ya", 'ゅ' => "yu", 'ょ' => "yo", 'っ' => "-" // "-" in the reading part means there is double consonant. Until I found better to do it.
);
pub const KATAKANA_ROMANJI: Map<char, &'static str> = phf_map!(
    'ア' => "a", 'イ' => "i", 'ウ' => "u", 'エ' => "e", 'オ' => "o",
    'カ' => "ka", 'キ' => "ki", 'ク' => "ku", 'ケ' => "ke", 'コ' => "ko",
    'サ' => "sa", 'シ' => "shi", 'ス' => "su", 'セ' => "se", 'ソ' => "so",
    'タ' => "ta", 'チ' => "chi", 'ツ' => "tsu", 'テ' => "te", 'ト' => "to",
    'ナ' => "na", 'ニ' => "ni", 'ヌ' => "nu", 'ネ' => "ne", 'ノ' => "no",
    'ハ' => "ha", 'ヒ' => "hi", 'フ' => "fu", 'ヘ' => "he", 'ホ' => "ho",
    'マ' => "ma", 'ミ' => "mi", 'ム' => "mu", 'メ' => "me", 'モ' => "mo",
    'ヤ' => "ya", 'ユ' => "yu", 'ヨ' => "yo",
    'ラ' => "ra", 'リ' => "ri", 'ル' => "ru", 'レ' => "re", 'ロ' => "ro",
    'ワ' => "wa", 'ヰ' => "wi", 'ヱ' => "we", 'ヲ' => "wo/o",
    'ン' => "n/m",
    'ガ' => "ga", 'ギ' => "gi", 'グ' => "gu", 'ゲ' => "ge", 'ゴ' => "go",
    'ザ' => "za", 'ジ' => "ji", 'ズ' => "zu", 'ゼ' => "ze", 'ゾ' => "zo",
    'ダ' => "da", 'ヂ' => "di", 'ヅ' => "du",	'デ' => "de", 'ド' => "do",
    'バ' => "ba", 'ビ' => "bi", 'ブ' => "bu", 'ベ' => "be", 'ボ' => "bo",
    'パ' => "pa", 'ピ' => "pi", 'プ' => "pu", 'ペ' => "pe", 'ポ' => "po",
    'ャ' => "ya", 'ュ' => "yu", 'ョ' => "yo", 'ッ' => "-"
);

/// all the available commands
pub const QUIT: [&str; 2] = ["exit", "e"];
pub const CLEAR: [&str; 2] = ["clear", "c"];
pub const HELP: [&str; 3] = ["help", "h", "?"];
pub const WORD: [&str; 2] = ["search", "s"];
pub const PLAY: [&str; 2] = ["play", "p"];
pub const EXPLAIN: [&str; 2] = ["explain", "x"];
pub const READING: [&str; 2] = ["reads", "r"];
pub const DEFINITION: [&str; 2] = ["define", "d"];

/// Help sentences
pub const USE_WORD: &str = "search [or s] (WORD: a word to fetch from the dictianory api.)
                            examples:
                            \tsearch apple
                            \ts house
";
pub const USE_PLAY: &str = "play [or p] (NUMBER: number of the word to get spoken. or. WORD: a word of your chose to get played.)
                            examples:
                            \tplay 1
                            \tplay 林檎";
pub const USE_EXPLAIN: &str = "explain [or x] (NUMBER,NUMBER,...: a number or more of the wanted explanations. or. all: to show all the explanations.)
                        examples:
                        \texplain all
                        \tx 1,2,3";
pub const USE_DEFINITION: &str = "define [or d] (NUMBER,NUMBER,...: a number or more of the wanted defined. or. all: to show all the definitions.)
                        examples:
                        \td all
                        \tdefine 1,2,3 all";
pub const USE_READING: &str = "reads [or r] (NUMBER,NUMBER,...: a number or more of the word to get defined. or. all: to show all the readings.)
                        examples:
                        \tr all
                        PhfHash\treads 1,2,3 all";
