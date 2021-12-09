#[repr(C)]
struct MagicAdder {
	amount: u32
}

impl MagicAdder { /* Snip... */ }

#[no_mangle]
extern "C" fn magicadder_new(amount: u32) -> MagicAdder {
	MagicAdder::new(amount)
}

#[no_mangle]
extern "C" fn magicadder_process_value(adder: *const MagicAdder, value: u32) -> u32 {
	if let Some(ma) = unsafe { adder.as_ref() } {
		ma.process_value(value)
	} else {
		0
	}
}
