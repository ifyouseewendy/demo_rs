use std::boxed::Box;
mod meta;

#[repr(C)]
pub struct LineItem {
	id: i32,
}

#[repr(C)]
pub struct DiscountCode {
	id: i32,
}

#[repr(C)]
pub struct Customer {
	id: u64,
}

#[repr(C)]
pub struct Checkout<'a> {
	line_items: &'a MySlice<'a, LineItem>,
	discount_codes: &'a MySlice<'a, DiscountCode>,
	customer: Option<&'a Customer>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Discount {
	id: i32,
}

#[repr(C)]
pub struct Input<'a> {
	v_str: &'a MySlice<'a, &'a str>,
	v_slice: &'a MySlice<'a, i32>,
	v_int: i32,
}

#[repr(C)]
pub struct MySlice<'a, T> {
	p: &'a T,
	len: u32,
}

#[no_mangle]
// pub extern "C" fn run<'a>(input: &'a Checkout) -> &'a MySlice<'a, Discount> {
pub extern "C" fn run<'a>(
	input: &'a Checkout,
) -> &'a MySlice<'a, &'a Discount> {
	let discounts =
		vec![&Discount { id: 2 }, &Discount { id: 2 }].into_boxed_slice();

	let p: &'a mut [&Discount] = Box::leak(discounts);

	let b = Box::new(MySlice { p: &p[0], len: 2 });
	Box::leak(b)
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
