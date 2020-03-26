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
pub struct Discount<'a> {
	id: i32,
	v_str: &'a MySlice<'a, u8>,
}

// impl<'a> Discount<'a> {
// 	pub fn new(id: i32, v_str: &str) _{
// 		Box::new(Discount { id, v_str: v_str.to_owned() })
// 	}
// }

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
pub extern "C" fn run<'a>(
	input: &'a Checkout,
) -> &'a MySlice<'a, &'a Discount<'a>> {
	let v_str = wrap_string("hello");
	transform(vec![Box::new(Discount { id: 2, v_str })])
}

fn transform<'a>(
	v: Vec<Box<Discount<'a>>>,
) -> &'a MySlice<'a, &'a Discount<'a>> {
	let vs = v
		.into_iter()
		.map(|b| &*Box::leak(b))
		.collect::<Vec<&Discount<'a>>>();
	wrap_slice(&vs)
}

fn wrap_slice<'a, T: Clone>(v: &[T]) -> &'a MySlice<'a, T> {
	let len = v.len() as u32;
	let p: &'a [T] = Box::leak(v.to_vec().into_boxed_slice());
	let b = Box::new(MySlice { p: &p[0], len });
	Box::leak(b)
}

fn wrap_string<'a>(s: &'a str) -> &'a MySlice<'a, u8> {
	let len = s.bytes().len() as u32;
	let chars = s.bytes().collect::<Vec<u8>>().into_boxed_slice();
	let c: &'a mut [u8] = Box::leak(chars);
	let b = Box::new(MySlice { p: &c[0], len });
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
