use std::str::Chars;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn rustlatin(sentence: &str) -> Vec<String> {
    let mut collection_of_words = Vec::new();
    
    for word in sentence.split(' ') {
        collection_of_words.push(word.to_owned() + "rs")
       
    };
    collection_of_words
}


#[test]
fn concatenated(){
    assert_eq!( vec!["dors", "yours", "likers", "rustrs"], rustlatin("do you like rust")) 
}

