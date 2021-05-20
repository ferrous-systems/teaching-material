pub mod phonebook {
    include!(concat!(env!("OUT_DIR"), "/phone.phonebook.rs"));
}

pub mod config {
    include!(concat!(env!("OUT_DIR"), "/phone.config.rs"));
}
