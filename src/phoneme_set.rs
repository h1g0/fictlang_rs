use rand::Rng;

#[derive(Debug, Clone, Copy)]
enum PhonologicalPattern {
    V,
    VC,
    CV,
    CVC,
}
#[derive(Debug, Clone, Copy)]
struct Phoneme {
    pub pattern: PhonologicalPattern,
    pub probability: f32,
}

struct PhonemeSet {
    pub phonemes: Vec<Phoneme>,
    pub total_probability: f32,
}
impl PhonemeSet {
    pub fn new() -> PhonemeSet {
        let phonemes = vec![
            Phoneme {
                pattern: PhonologicalPattern::V,
                probability: 0.25,
            },
            Phoneme {
                pattern: PhonologicalPattern::VC,
                probability: 0.25,
            },
            Phoneme {
                pattern: PhonologicalPattern::CV,
                probability: 0.25,
            },
            Phoneme {
                pattern: PhonologicalPattern::CVC,
                probability: 0.25,
            },
        ];
        let total_probability = phonemes.iter().map(|p| p.probability).sum();
        PhonemeSet {
            phonemes,
            total_probability,
        }
    }
    pub fn get_random(&self) -> Phoneme {
        let mut rand = rand::thread_rng().gen_range(0.0..self.total_probability);
        for p in self.phonemes.iter() {
            if rand < p.probability {
                return *p;
            }
            rand -= p.probability;
        }
        panic!("Should not happen");
    }
}
