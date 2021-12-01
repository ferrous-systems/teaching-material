Unsafe, FFI, and Bindings

* Safe binding, both manually and automatically
* Binding strategies
* The role of unsafe
* The scope of unsafe
* Working with raw pointers and helping pointers
* Do’s and Don’ts of unsafe Rust
* Introduction into support APIs, like non-null pointers
* Potential undefined behaviour arising from the use of unsafe
* Checking unsafe Rust for safety
* Using Rusts guarantees for security gains

James:

What I want to expose:
* single function
* takes a C string of "input"
* returns (if successful) the output

parse_and_eval

Arguments?
   * const char
Return type?
   * integer

returns 0 if success, returns nonzero if failure
on success, output is updated with the result

int parse_and_eval(char*, int64_t* output);
