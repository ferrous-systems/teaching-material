#[repr(C)]
pub struct MagicAdder {
    amount: u32
}

impl MagicAdder {
    pub fn new(amount: u32) -> MagicAdder {
        MagicAdder {
            amount
        }
    }

    pub fn process_value(&self, value: u32) -> u32 {
        self.amount + value
    }
}

#[no_mangle]
pub extern "C" fn magicadder_new(amount: u32) -> MagicAdder {
    MagicAdder::new(amount)
}

#[no_mangle]
pub extern "C" fn magicadder_process_value(adder: *const MagicAdder, value: u32) -> u32 {
    if let Some(ma) = unsafe { adder.as_ref() } {
        ma.process_value(value)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ma = MagicAdder::new(5);
        assert_eq!(6, ma.process_value(1));
        assert_eq!(10, ma.process_value(5));
    }
}
