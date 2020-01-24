trait Named {
    fn name(&self) -> &'static str;
}

trait Person : Named {
    fn home_address(&self) -> Address;
}