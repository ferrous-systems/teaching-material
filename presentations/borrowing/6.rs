fn main() {
    let mut data = vec!['a', 'b', 'c'];
    {
        let slice = &mut data[..];
        do_something(slice);
    }
    data.push('d');
    data.push('e');
    data.push('f');
}

fn do_something(slice: &[char]) {

}