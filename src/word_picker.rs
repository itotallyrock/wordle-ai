use arrayvec::ArrayVec;

use crate::game::NUM_LETTERS;

include!(concat!(env!("OUT_DIR"), "/word_list.rs"));

#[derive(Debug)]
pub struct WordPicker {
    remaining_words: ArrayVec<&'static str, WORD_LIST_LEN>,
}

impl WordPicker {
    pub fn new() -> Self {
        let mut words = Self {
            remaining_words: ArrayVec::from(WORD_LIST),
        };

        // Shuffle word list to start so we get some random input
        fastrand::shuffle(words.remaining_words.as_mut_slice());
        // Sort words with most unique characters towards end
        words
            .remaining_words
            .sort_by_cached_key(|word| unique_letters_per_word(word));

        words
    }
    pub fn remove_words_containing(&mut self, illegal_letter: char) {
        debug_assert!(
            illegal_letter.is_ascii_alphabetic(),
            "letter must be legal ascii letter"
        );
        debug_assert!(illegal_letter.is_lowercase(), "letter must be lowercase");

        self.remaining_words
            .retain(|word| !word.contains(illegal_letter));
    }
    pub fn remove_words_without_letter_in_position(
        &mut self,
        required_letter: char,
        required_position: usize,
    ) {
        debug_assert!(
            required_position < NUM_LETTERS,
            "cannot remove words with letter in position that is longer than {}",
            NUM_LETTERS
        );
        debug_assert!(
            required_letter.is_ascii_alphabetic(),
            "letter must be legal ascii letter"
        );
        debug_assert!(required_letter.is_lowercase(), "letter must be lowercase");

        self.remaining_words
            .retain(|word| word.chars().nth(required_position) == Some(required_letter));
    }
    pub fn remove_words_not_containing(&mut self, required_letter: char) {
        debug_assert!(
            required_letter.is_ascii_alphabetic(),
            "letter must be legal ascii letter"
        );
        debug_assert!(required_letter.is_lowercase(), "letter must be lowercase");

        self.remaining_words
            .retain(|word| word.contains(required_letter));
    }
    pub fn pick_best_word(&mut self) -> Option<&'static str> {
        self.remaining_words.pop()
    }
    pub fn pick_random(&mut self) -> Option<&'static str> {
        self.remaining_words
            .pop_at(fastrand::usize(0..self.remaining_words.len()))
    }
}

fn unique_letters_per_word(word: &str) -> usize {
    const ALPHA_LEN: usize = 26;
    let mut seen = [0u8; ALPHA_LEN];

    debug_assert_eq!(
        word.to_ascii_lowercase(),
        word,
        "cannot get unique letters for non-lowercase words"
    );

    for c in word.chars() {
        debug_assert!(
            c >= 'a' && c <= 'z',
            "illegal letter in unique_letters_per_word"
        );
        let letter_index = ((c as u8) - ('a' as u8)) as usize;
        seen[letter_index] = 1;
    }

    seen.into_iter().sum::<u8>() as usize
}
