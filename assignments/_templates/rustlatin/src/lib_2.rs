const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn rustlatin(sentence: &str) -> Vec<T> {
                            // ^^^^^^^ The correct return type needs to be added by you, 
                            //         depending on what the vector's exact type is. 
    let mut collection_of_words = Vec::new();
    
    for word in sentence.split(' ') {
        // Your implementation goes here:
       
    };
    collection_of_words
}

#[test]
fn concatenated(){
    assert_eq!( vec!["dors", "yours", "likers", "rustrs"], rustlatin("do you like rust")) 
}

