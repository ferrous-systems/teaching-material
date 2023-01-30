const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn rustlatin(sentence: &str) -> Vec<&str> {
    let mut collection_of_words = Vec::new();
    
    for word in sentence.split(' ') {
        collection_of_words.push(word);
    };
    
    collection_of_words
}


#[test]
fn correct_splitting(){
    assert_eq!(vec!["This", "sentence", "needs", "to", "be", "split"], rustlatin("This sentence needs to be split"))

}
