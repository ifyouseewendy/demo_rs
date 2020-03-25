mod meta;

#[repr(C)]
pub struct LineItem {
	id: String,
}

#[repr(C)]
pub struct DiscountCode {
	id: String,
}

#[repr(C)]
pub struct Customer {
	id: u64,
	email: String,
	tags: Vec<String>,
}

#[repr(C)]
pub struct Checkout {
	line_items: Vec<LineItem>,
	discount_codes: Vec<DiscountCode>,
	customer: Option<Customer>,
}

#[repr(C)]
#[derive(Debug)]
pub struct Discount {
	line_item_id: String,
	value: i32,
	title: String,
}

#[repr(C)]
pub struct Input<'a> {
	v_str: &'a MySlice<'a, &'a str>,
	v_slice: &'a MySlice<'a, i32>,
	v_int: i32,
}

pub struct MySlice<'a, T> {
	p: &'a T,
	len: u32,
}

#[no_mangle]
pub extern "C" fn run(input: &Input) -> u32 {
	input.v_slice.len
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_run() {
		let line_items: Vec<LineItem> = vec![];
		let customer = Customer {
			id: 1,
			email: "di@example.com".to_owned(),
			tags: Vec::new(),
		};
		let discount_codes: Vec<DiscountCode> = Vec::new();
		let input = Checkout {
			line_items,
			discount_codes,
			customer: Some(customer),
		};
		assert_eq!(run(&input).len(), 0);
	}
}
