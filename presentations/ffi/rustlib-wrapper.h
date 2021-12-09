/// Designed to have the exact same shape as the Rust version
typedef struct magic_adder_t {
	uint32_t amount;
} magic_adder_t;

/// Wraps MagicAdder::new
magic_adder_t magicadder_new(uint32_t amount);

/// Wraps MagicAdder::process_value
uint32_t magicadder_process_value(magic_adder_t* self, uint32_t value);
