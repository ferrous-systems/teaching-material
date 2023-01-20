const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
// ^^^^^^^^^ The vowls are contained in an array, because the length never changes. 
//           It's a global const because it will not be modified in any way and only
//           serves as a reference.

fn rustlatin(sentence: &str) -> Vec<T> {
                            // ^^^^^^^ The correct return type needs to be added by you, 
                            //         depending on what the vector's exact type is. 

    let mut collection_of_words = Vec::new(); 
                              // ^^^^^^^^^^^^ When you first open this file RA is not able to infer 
                              //              the type of this vector. Once you do the implementation, 
                              //              the type should appear here automatically.
    
    // Your implementation goes here:

    
    collection_of_words
}


#[test]
fn correct_splitting(){
    assert_eq!(vec!["This", "sentence", "needs", "to", "be", "split"], rustlatin("This sentence needs to be split"))

}

