let check_example = is_website_up("http://example.com");
let check_forever = is_website_up("http://httpforever.com");

let (example_result, forever_result) = join!(check_example, check_forever);
