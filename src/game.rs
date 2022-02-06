use arrayvec::ArrayVec;

use crate::WORD_LIST;

pub const NUM_LETTERS: usize = 5;
pub const MAX_GUESSES: usize = 6;

pub struct Game {
    solution_str: &'static str,
    solution_chars: [char; NUM_LETTERS],
    guesses: ArrayVec<String, MAX_GUESSES>,
}

#[derive(PartialEq, Debug)]
pub enum GuessError {
    TooManyGuesses,
    IllegalWord,
    AlreadySolved,
}

#[derive(PartialEq, Debug)]
pub enum GuessResult {
    Miss(char),
    Partial(char),
    Hit(char),
}

impl Game {
    pub fn new(solution: &'static str) -> Self {
        assert_eq!(
            solution.len(),
            NUM_LETTERS,
            "Expected a solution to have {} letters",
            NUM_LETTERS
        );
        assert!(WORD_LIST.contains(&solution), "Word not found in word list");

        Self {
            solution_str: solution,
            solution_chars: solution.chars().collect::<Vec<_>>().try_into().unwrap(),
            guesses: ArrayVec::new(),
        }
    }
    pub fn try_guess(&mut self, guess: &str) -> Result<[GuessResult; NUM_LETTERS], GuessError> {
        if self.is_solved() {
            return Err(GuessError::AlreadySolved);
        }
        if self.remaining_guesses() == 0 {
            return Err(GuessError::TooManyGuesses);
        }
        // TODO: Use larger dictionary to check word legality instead of just length
        if guess.len() != NUM_LETTERS {
            return Err(GuessError::IllegalWord);
        }

        self.guesses.push(guess.to_ascii_lowercase());

        Ok(guess
            .chars()
            .zip(self.solution_chars.iter())
            .map(|(guess_char, &solution_char)| {
                if guess_char == solution_char {
                    GuessResult::Hit(guess_char)
                } else if self.solution_str.contains(guess_char) {
                    GuessResult::Partial(guess_char)
                } else {
                    GuessResult::Miss(guess_char)
                }
            })
            .collect::<ArrayVec<GuessResult, NUM_LETTERS>>()
            .into_inner()
            .ok()
            .unwrap())
    }
    pub fn is_solved(&self) -> bool {
        self.guesses.last().map(|s| s.as_str()) == Some(self.solution_str)
    }
    pub fn remaining_guesses(&self) -> usize {
        MAX_GUESSES - self.guesses.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn game_new_works() {
        Game::new(WORD_LIST[0]);
    }

    #[test]
    fn game_guess_hit_miss_works() {
        let mut game = Game::new("those");
        assert_eq!(
            game.try_guess("troll"),
            Ok([
                GuessResult::Hit('t'),
                GuessResult::Miss('r'),
                GuessResult::Hit('o'),
                GuessResult::Miss('l'),
                GuessResult::Miss('l'),
            ])
        );
    }

    #[test]
    fn game_guess_all_miss_works() {
        let mut game = Game::new("weigh");
        assert_eq!(
            game.try_guess("troll"),
            Ok([
                GuessResult::Miss('t'),
                GuessResult::Miss('r'),
                GuessResult::Miss('o'),
                GuessResult::Miss('l'),
                GuessResult::Miss('l'),
            ])
        );
    }

    #[test]
    fn game_guess_mostly_partial_works() {
        let mut game = Game::new("hotel");
        assert_eq!(
            game.try_guess("lathe"),
            Ok([
                GuessResult::Partial('l'),
                GuessResult::Miss('a'),
                GuessResult::Hit('t'),
                GuessResult::Partial('h'),
                GuessResult::Partial('e'),
            ])
        );
    }
}
