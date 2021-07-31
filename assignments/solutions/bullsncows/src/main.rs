
use rand::prelude::*;

const NUM_DIGITS : usize = 4;

// Write this function.
//
// Hints:
//   Copy the guess and secrets arrays using Vec::from()
//   return (num_bulls, num_cows)
fn num_bulls_and_cows(guess: &[i32], secret: &[i32]) -> (i32, i32) {
    // Duplicate the secret, to we can tick off the ones we find.
    let mut secret = Vec::from(secret);
    let mut guess = Vec::from(guess);

    // Count the bulls (exact matches) first and tick them off.
    let mut num_bulls = 0;
    for (g, s) in guess.iter_mut().zip(secret.iter_mut()) {
        if *g == *s {
            num_bulls += 1;
            *s = 0;
            *g = -1;
        }
    }

    // Count the number of cows (inexact matches) and tick them off.
    let mut num_cows = 0;
    for g in guess {
        if let Some(pos) = secret.iter().position(|s| *s == g) {
            secret[pos] = 0;
            num_cows += 1;
        }
    }
    (num_bulls, num_cows)
}

fn main() {
    let mut rng = rand::thread_rng();

    let secret : Vec<_> = (0..NUM_DIGITS).map(|_| (rng.gen::<i32>() & 3) + 1 ).collect();

    let stdin = std::io::stdin();

    let mut buf = String::new();

    eprintln!("Bulls and Cows.");
    eprintln!("Four digits between have been 1 and 4 have been generated.");
    eprintln!("A 'bull' is a correct digit in the correct place.");
    eprintln!("A 'cow' is a correct digit in the incorrect place.");
    eprintln!("Guesses are only counted once.");
    eprintln!("A win is four bulls and no cows.");
    eprintln!("Example: with secret 1 3 3 2");
    eprintln!("guess: 1 2 3 4 -> bull cow bull nothing");
    eprintln!("guess: 2 2 2 2 -> cow nothing nothing nothing");
    eprintln!("guess: 3 2 2 3 -> cow nothing nothing cow");
    eprintln!("");

    // println!("secret = {:?}", secret);

    loop {
        buf.clear();
        eprint!("guess: ");
        stdin.read_line(&mut buf).unwrap();
        let guess : Result<Vec<i32>, _> = buf.trim().split(' ').map(|s| s.parse()).collect();

        if let Ok(guess) = guess {
            if guess.len() == NUM_DIGITS {
                let (bulls, cows) = num_bulls_and_cows(&guess, &secret);

                eprintln!("{:?} has {} bulls and {} cows", guess, bulls, cows);
        
                if bulls == 4 {
                    break;
                }
            } else {
                eprintln!("try a guess like: 1 2 3 4")
            }
        } else {
            eprintln!("try a guess like: 1 2 3 4")
        }

    }
    eprintln!("You win!");
}

#[test]
fn test_num_bulls_and_cows() {
    assert_eq!(num_bulls_and_cows(&[1, 2, 3, 4], &[1, 2, 3, 4]), (4, 0));
    assert_eq!(num_bulls_and_cows(&[1, 2, 3, 5], &[1, 2, 3, 4]), (3, 0));
    assert_eq!(num_bulls_and_cows(&[4, 3, 2, 1], &[1, 2, 3, 4]), (0, 4));
    assert_eq!(num_bulls_and_cows(&[1, 2, 3, 1], &[1, 2, 3, 4]), (3, 0));
    assert_eq!(num_bulls_and_cows(&[1, 1, 1, 1], &[1, 2, 3, 4]), (1, 0));
    assert_eq!(num_bulls_and_cows(&[1, 2, 2, 2], &[2, 2, 2, 1]), (2, 2));
    assert_eq!(num_bulls_and_cows(&[1, 3, 3, 2], &[2, 2, 2, 1]), (0, 2));
}
