use crate::ai::Ai;
use crate::game::Game;
use crate::word_picker::WORD_LIST;

mod ai;
mod game;
mod word_picker;

const BENCH_RUNS: usize = 1000;

fn main() {
    let mut total_losses = 0;
    let mut total_wins = 0;
    for _ in 0..BENCH_RUNS {
        let mut wins = 0;
        let mut losses = 0;
        for &word in &WORD_LIST {
            let mut solver = Ai::new();
            let mut game = Game::new(word);

            while !game.is_solved() && game.remaining_guesses() > 0 {
                solver.guess(&mut game).expect("unable to guess");
            }

            if game.is_solved() {
                // println!("solved {}", word);
                wins += 1;
            } else {
                println!("failed to solve {}", word);
                losses += 1;
            }
        }
        total_losses += losses;
        total_wins += wins;

        println!(
            "won {:3.0}% {} / {}",
            (wins as f32 / (wins + losses) as f32) * 100f32,
            wins,
            losses,
        );
    }
    println!(
        "totally won {:3.0}% {} / {}",
        (total_wins as f32 / (total_wins + total_losses) as f32) * 100f32,
        total_wins,
        total_losses,
    );
}
