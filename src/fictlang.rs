use std::collections::HashMap;
use rand::Rng;

pub struct CharSet{
    vowels: HashMap<char,f32>,
    consonants: HashMap<char,f32>,
    vowel_total_probability: f32,
    consonant_total_probability: f32,
}

impl CharSet{
    pub fn new() -> CharSet{
        let mut vowels = HashMap::new();
        let mut consonants = HashMap::new();

        // cf. https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_letters_in_other_languages
        vowels.insert('a', 6.516);
        vowels.insert('e', 16.396);
        vowels.insert('i', 6.550);
        vowels.insert('o', 2.594);
        vowels.insert('u', 4.166);
        vowels.insert('ä', 0.578);
        vowels.insert('ö', 0.443);
        vowels.insert('ü', 0.995);

        consonants.insert('b', 1.886);
        consonants.insert('c', 2.732);
        consonants.insert('d', 5.076);
        consonants.insert('f', 1.656);
        consonants.insert('g', 3.009);
        consonants.insert('h', 4.577);
        consonants.insert('j', 0.268);
        consonants.insert('k', 1.417);
        consonants.insert('l', 3.437);
        consonants.insert('m', 2.534);
        consonants.insert('n', 9.776);
        consonants.insert('p', 0.670);
        consonants.insert('q', 0.018);
        consonants.insert('r', 7.003);
        consonants.insert('s', 7.270);
        consonants.insert('t', 6.154);
        consonants.insert('v', 0.846);
        consonants.insert('w', 1.921);
        consonants.insert('x', 0.034);
        consonants.insert('y', 0.039);
        consonants.insert('z', 1.134);
        consonants.insert('ß', 0.307);

        let mut vowel_total_probability = 0.0;
        let mut consonant_total_probability = 0.0;

        for (_, probability) in vowels.iter() {
            vowel_total_probability += probability;
        }
        for (_, probability) in consonants.iter() {
            consonant_total_probability += probability;
        }

        return CharSet{
            vowels,
            consonants,
            vowel_total_probability,
            consonant_total_probability,
        };
    }

    fn get_random_vowel(&self) -> char {
        let mut random_number = rand::thread_rng().gen_range(0.0..self.vowel_total_probability);
        for (vowel, probability) in self.vowels.iter() {
            if random_number < *probability {
                return *vowel;
            }
            random_number -= probability;
        }
        panic!("No vowel found");
    }
    fn get_random_consonant(&self) -> char {
        let mut random_number = rand::thread_rng().gen_range(0.0.. self.consonant_total_probability);
        for (consonant, probability) in self.consonants.iter() {
            if random_number < *probability {
                return *consonant;
            }
            random_number -= *probability;
        }
        panic!("No consonant found");
    }

    fn get_random_word(&self) -> String {
        let mut word = String::new();

        loop{
            let phonological_pattern_no = rand::thread_rng().gen_range(0..4);
            match phonological_pattern_no {
                0 => { // V
                    word.push(self.get_random_vowel());
                },
                1 => { // CV
                    word.push(self.get_random_consonant());
                    word.push(self.get_random_vowel());
                },
                2 => { // VC
                    word.push(self.get_random_vowel());
                    word.push(self.get_random_consonant());
                },
                3 => { // CVC
                    word.push(self.get_random_consonant());
                    word.push(self.get_random_vowel());
                    word.push(self.get_random_consonant());
                },
                _ => {}
            }
            if rand::thread_rng().gen_range(0..3) == 0 {
                break;
            }
        }
        return word;
    }

    pub fn get_random_sentence(&self) -> String {
        let mut sentence = Vec::new();
        loop{
            sentence.push(self.get_random_word());
            let terminator = rand::thread_rng().gen_range(0.0.. 1.0);
            if terminator < 0.2 {
                if terminator < 0.06 {
                    sentence.push(".".to_string());
                } else if terminator < 0.1{
                    sentence.push("?".to_string());
                } else if terminator < 0.12{
                    sentence.push("!".to_string());
                } else {
                    sentence.push(",".to_string());
                }
                break;
            }else {
                sentence.push(" ".to_string());
            }

        }
        return sentence.join("");
    }

}
