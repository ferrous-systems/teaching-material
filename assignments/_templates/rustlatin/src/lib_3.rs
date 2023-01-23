const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn rustlatin(sentence: &str) -> Vec<_> {
                            // ^^^^^^^ The correct return type needs to be added by you, 
                            //         depending on what the vector's exact type is. 
    let mut collection_of_chars = Vec::new();
    
    for word in sentence.split(' ') {
        // Your implementation goes here:
        // Add the first char of each word to the vector. 
        // Correct the return type of the vector.

    };
    collection_of_chars
}


#[test]
fn return_the_char(){
    assert_eq!(vec!['n', 't', 'd', 'b', 'i', 'a', 'r', 'v'], rustlatin("note the difference between iterator and return values"))
}

