fn doesnt_return() {
    true;
}

fn doesnt_return2() -> () {
    true;
    ()
}

fn doesnt_compile() -> bool {
    true;
}

fn returns() -> bool {
    true
}
