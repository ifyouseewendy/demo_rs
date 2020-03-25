mod meta;

#[no_mangle]
pub extern "C" fn run(input: u32) -> u32 {
	input + 1
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_run() {
		assert_eq!(run(1), 2);
	}
}
