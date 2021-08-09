
pub struct Prefix {
    pub conjugation: Vec<String>,
}
pub struct PrefixSet {
    pub prefixes: Vec<Prefix>,
}
pub struct Suffix {
    pub conjugation: Vec<String>,
}
pub struct SuffixSet {
    pub suffixes: Vec<Suffix>,
}
pub struct Word {
    prefix_idx: Option<usize>,
    suffix_idx: Option<usize>,
    str: String,
}
