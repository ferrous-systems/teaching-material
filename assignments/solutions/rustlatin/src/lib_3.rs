use std::str::Chars;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn rustlatin(sentence: &str) -> Vec<char> {
    let mut collection_of_words = Vec::new();
    
    for word in sentence.split(' ') {
        let first_char = word.chars().next().unwrap();
        collection_of_words.push(first_char);
    };
    words
 
}

#[test]
fn return_the_char(){
    assert_eq!(vec!['n', 't', 'd', 'b', 'i', 'a', 'r', 'v'], rustlatin("note the difference between iterator and return values"))
}

