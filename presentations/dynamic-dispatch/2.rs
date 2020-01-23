enum Operation {
    Get,
    Set(String),
    Count
}

impl Operation {
    fn execute(&self) {
        match &self {
            &Operation::Get => execute_get(),
            &Operation::Set(s) => execute_set(s),
            &Operation::Count => execute_count()
        }
    }
}