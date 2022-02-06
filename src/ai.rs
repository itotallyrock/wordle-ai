use crate::game::{Game, GuessError, GuessResult, MAX_GUESSES};
use crate::word_picker::WordPicker;

#[derive(Debug)]
pub struct Ai {
    word_picker: WordPicker,
    remaining_guesses: usize,
}

impl Ai {
    pub fn new() -> Self {
        Self {
            word_picker: WordPicker::new(),
            remaining_guesses: MAX_GUESSES,
        }
    }
    pub fn next_guess(&mut self) -> &'static str {
        // If we're on our last two guesses choose a random word instead of one with the most unique characters
        if self.remaining_guesses <= 3 {
            self.word_picker
                .pick_random()
                .expect("failed to get final guess")
        } else {
            self.word_picker
                .pick_best_word()
                .expect("failed to get guess")
        }
    }
    pub fn guess(&mut self, game: &mut Game) -> Result<(), GuessError> {
        let attempt = self.next_guess();
        let result = game.try_guess(attempt)?;

        for (char_index, guess_result) in result.into_iter().enumerate() {
            match guess_result {
                GuessResult::Hit(required_char) => self
                    .word_picker
                    .remove_words_without_letter_in_position(required_char, char_index),
                GuessResult::Partial(partial_char) => {
                    self.word_picker.remove_words_not_containing(partial_char)
                }
                GuessResult::Miss(illegal_char) => {
                    self.word_picker.remove_words_containing(illegal_char)
                }
            }
        }

        Ok(())
    }
}
