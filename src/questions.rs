pub struct Question<'a> {
    pub character: char,
    pub answer: &'a str,
}

pub const QUESTIONS: [Question; 46] = [
    Question { character: 'あ', answer: "a" },
    Question { character: 'い', answer: "i" },
    Question { character: 'う', answer: "u" },
    Question { character: 'え', answer: "e" },
    Question { character: 'お', answer: "o" },

    Question { character: 'か', answer: "ka" },
    Question { character: 'き', answer: "ki" },
    Question { character: 'く', answer: "ku" },
    Question { character: 'け', answer: "ke" },
    Question { character: 'こ', answer: "ko" },

    Question { character: 'さ', answer: "sa" },
    Question { character: 'し', answer: "shi" },
    Question { character: 'す', answer: "su" },
    Question { character: 'せ', answer: "se" },
    Question { character: 'そ', answer: "so" },

    Question { character: 'た', answer: "ta" },
    Question { character: 'ち', answer: "chi" },
    Question { character: 'つ', answer: "tsu" },
    Question { character: 'て', answer: "te" },
    Question { character: 'と', answer: "to" },

    Question { character: 'な', answer: "na" },
    Question { character: 'に', answer: "ni" },
    Question { character: 'ぬ', answer: "nu" },
    Question { character: 'ね', answer: "ne" },
    Question { character: 'の', answer: "no" },

    Question { character: 'は', answer: "ha" },
    Question { character: 'ひ', answer: "hi" },
    Question { character: 'ふ', answer: "fu" },
    Question { character: 'へ', answer: "he" },
    Question { character: 'ほ', answer: "ho" },

    Question { character: 'ま', answer: "ma" },
    Question { character: 'み', answer: "mi" },
    Question { character: 'む', answer: "mu" },
    Question { character: 'め', answer: "me" },
    Question { character: 'も', answer: "mo" },

    Question { character: 'や', answer: "ya" },
    Question { character: 'ゆ', answer: "yu" },
    Question { character: 'よ', answer: "yo" },

    Question { character: 'ら', answer: "ra" },
    Question { character: 'り', answer: "ri" },
    Question { character: 'る', answer: "ru" },
    Question { character: 'れ', answer: "re" },
    Question { character: 'ろ', answer: "ro" },

    Question { character: 'わ', answer: "wa" },
    Question { character: 'を', answer: "wo" },

    Question { character: 'ん', answer: "n" },
];
