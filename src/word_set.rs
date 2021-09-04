use rand::Rng;

pub struct Prefix {
    pub conjugation: Vec<String>,
}
impl Prefix {
    pub fn new(conjugation: Vec<String>) -> Prefix {
        Prefix { conjugation }
    }
    pub fn get_random(&self) -> String {
        self.conjugation
            .get(rand::thread_rng().gen_range(0..self.conjugation.len() as usize))
            .unwrap()
            .clone()
    }
}
pub struct PrefixSet {
    pub prefixes: Vec<Prefix>,
}
impl PrefixSet {
    pub fn new(prefixes: Vec<Prefix>) -> PrefixSet {
        PrefixSet { prefixes }
    }
}

pub struct Suffix {
    pub conjugation: Vec<String>,
}
impl Suffix {
    pub fn new(conjugation: Vec<String>) -> Suffix {
        Suffix { conjugation }
    }
    pub fn get_random(&self) -> String {
        self.conjugation
            .get(rand::thread_rng().gen_range(0..self.conjugation.len() as usize))
            .unwrap()
            .clone()
    }
}
pub struct SuffixSet {
    pub suffixes: Vec<Suffix>,
}
impl SuffixSet {
    pub fn new(suffixes: Vec<Suffix>) -> SuffixSet {
        SuffixSet { suffixes }
    }
}

pub struct Word {
    prefix_idx: Option<usize>,
    suffix_idx: Option<usize>,
    str: String,
}
impl Word {
    pub fn new(prefix_idx: Option<usize>, suffix_idx: Option<usize>, str: String) -> Word {
        Word {
            prefix_idx,
            suffix_idx,
            str,
        }
    }
}
pub struct WordSet {
    pub words: Vec<Word>,
}
impl WordSet {
    pub fn new(words: Vec<Word>) -> WordSet {
        WordSet { words }
    }
}
