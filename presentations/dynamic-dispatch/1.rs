enum Operation {
    Get,
    Set(String),
    Count
}

fn execute(op: Operation) {
    match op {
        Operation::Get => execute_get(),
        Operation::Set(s) => execute_set(s),
        Operation::Count => execute_count()
    }
}