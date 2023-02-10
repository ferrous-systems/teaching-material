use std::fs::File;


fn main() {
    let _open_result = File::open("src/lib/content.txt").unwrap();
                                                                // ^^^^^^^^ File::open yields a Result<File, Error>, 
                                                                //  a quick way to get to the File type is to use 
                                                                // the unwrap() method on the Result<File, Error>. 
                                                               

    // ...
}


