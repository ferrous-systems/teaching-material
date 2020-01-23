#[repr(C)]
pub struct myapp_t {
    id: u32,
    other_type: Box<myother_t>,
    initialised: bool,
}