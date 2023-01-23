
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn rustlatin(sentence: &str) -> String {

    let mut collection_of_words = Vec::new();
    
    for word in sentence.split(' ') {
        let first_char = word.chars().next().unwrap();
        // Your implementation goes here
        // pushes the latinized words into the vector
    };
    collection_of_words.join(" ")
}

// fn latinize() goes here
// adds prefix "sr" and suffix "rs" according to the rules



#[test]
fn test_latinizer() {
    assert_eq!(latinize("rust"), "rustrs");
    assert_eq!(latinize("helps"), "helpsrs");
    assert_eq!(latinize("you"), "yours");
    assert_eq!(latinize("avoid"), "sravoid");
    
}

#[test]
fn correct_translation() {
    // Why can we compare `&str` and `String` here?
    // https://doc.rust-lang.org/stable/std/string/struct.String.html#impl-PartialEq%3C%26%27a%20str%3E
    assert_eq!(
        "rustrs helpsrs yours sravoid sra lotrs srof srirritating bugsrs",
        rustlatin("rust helps you avoid a lot of irritating bugs")
    )
}

#[test]
fn incorrect() {
    assert_ne!(
        "this shouldrs not workrs",
        rustlatin("this should not work")
    )
}
