#[serde(deny_unknown_fields)] // Be extra strict
struct Move {
    #[serde(default)] // Call usize::default()
    id: usize,
    #[serde(rename = "dir")] // Use a different name
    direction: Direction,
}