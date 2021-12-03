struct MagicAdder {
	amount: u32
}

impl MagicAdder {
	fn new(amount: u32) -> MagicAdder {
		MagicAdder {
			amount
		}
	}

	fn process_value(&self, value: u32) -> u32 {
		self.amount + value
	}
}
