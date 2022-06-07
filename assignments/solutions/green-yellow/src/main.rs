use rand::prelude::*;

const NUM_DIGITS: usize = 4;

const GREEN: &'static str = "🟩";
const YELLOW: &'static str = "🟨";
const BLANK: &'static str = "⬜";

// Write this function.
//
// Hints:
//   Copy the guess and secrets arrays using Vec::from() so you can mutate them
//   return a seven-character String with four coloured blocks and three spaces
fn calc_green_and_yellow(guess: &[i32], secret: &[i32]) -> String {
    // Duplicate the secret, to we can tick off the ones we find.
    let mut secret = Vec::from(secret);
    let mut guess = Vec::from(guess);

    let mut result: [&'static str; 4] = [BLANK, BLANK, BLANK, BLANK];

    // Count the green squares (right number, right place) first and tick them off.
    for ((g, s), r) in guess
        .iter_mut()
        .zip(secret.iter_mut())
        .zip(result.iter_mut())
    {
        if *g == *s {
            *s = 0;
            *g = -1;
            *r = GREEN;
        }
    }

    // Count the number of yellow squares (right number, wrong place) and tick them off.
    for (g, r) in guess.iter().zip(result.iter_mut()) {
        if let Some(pos) = secret.iter().position(|s| *s == *g) {
            secret[pos] = 0;
            if *r == BLANK {
                *r = YELLOW;
            }
        }
    }

    result.join(" ")
}

fn main() {
    let mut rng = rand::thread_rng();

    let secret: Vec<_> = (0..NUM_DIGITS).map(|_| rng.gen_range(1..=9)).collect();

    let stdin = std::io::stdin();

    let mut buf = String::new();

    println!("Green and Yellow");
    println!("(it's like Wordle with Numbers)");
    println!();
    println!("Four digits between 1 and 9 have been generated.");
    println!("A '{}' is a correct digit in the correct place.", GREEN);
    println!("A '{}' is a correct digit in the incorrect place.", YELLOW);
    println!("Guesses are only counted once.");
    println!("A win is four {} and no {} or {}.", GREEN, YELLOW, BLANK);
    println!("Example: with secret 1 2 3 4");
    println!("guess: 4 3 2 1 -> 🟨 🟨 🟨 🟨");
    println!("guess: 1 2 3 1 -> 🟩 🟩 🟩 ⬜");
    println!("guess: 1 1 1 2 -> 🟩 ⬜ ⬜ 🟨");
    println!("");

    // println!("secret = {:?}", secret);

    loop {
        buf.clear();
        eprint!("guess: ");
        stdin.read_line(&mut buf).unwrap();
        let guess: Result<Vec<i32>, _> = buf.trim().split(' ').map(|s| s.parse()).collect();

        if let Ok(guess) = guess {
            if guess.iter().filter(|x| **x <= 9).count() == NUM_DIGITS {
                let squares = calc_green_and_yellow(&guess, &secret);

                println!("{:?} gave {}", guess, squares);

                if squares == "🟩 🟩 🟩 🟩" {
                    break;
                }
            } else {
                println!("try a guess like: 1 2 3 4")
            }
        } else {
            println!("try a guess like: 1 2 3 4")
        }
    }
    println!("You win!");
}

#[test]
fn test_green_and_yellow() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]),
        "🟩 🟩 🟩 🟩".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]),
        "🟩 🟩 🟩 ⬜".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]),
        "🟨 🟨 🟨 🟨".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]),
        "🟩 🟩 🟩 ⬜".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 1, 1, 2], &[1, 2, 3, 4]),
        "🟩 ⬜ ⬜ 🟨".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]),
        "🟨 🟩 🟩 🟨".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]),
        "🟨 ⬜ ⬜ 🟨".to_string()
    );
}
