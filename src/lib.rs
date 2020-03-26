use std::boxed::Box;
mod meta;

/// Core structure which is bound to our memory layout
/// https://github.com/Shopify/runtime-engine/wiki/Memory-Layout
#[repr(C)]
pub struct MySlice<'a, T> {
	p: &'a T,
	len: u32,
}

#[repr(C)]
pub struct Input<'a> {
	v_int: i32,
	v_str: &'a MySlice<'a, u8>,
	v_slice: &'a MySlice<'a, i32>,
}

#[repr(C)]
pub struct Output<'a> {
	v_int: i32,
	v_str: &'a MySlice<'a, u8>,
	v_slice: &'a MySlice<'a, i32>,
}

#[no_mangle]
pub extern "C" fn run<'a>(input: &'a Input) -> &'a Output<'a> {
	let v_int = 42;
	let v_str = wrap_string("hello world");
	let v_slice = wrap_slice(&vec![1, 2, 3]);
	Box::leak(Box::new(Output {
		v_int,
		v_str,
		v_slice,
	}))
}

// fn transform<'a>(
// 	v: Vec<Box<Discount<'a>>>,
// ) -> &'a MySlice<'a, &'a Discount<'a>> {
// 	let vs = v
// 		.into_iter()
// 		.map(|b| &*Box::leak(b))
// 		.collect::<Vec<&Discount<'a>>>();
// 	wrap_slice(&vs)
// }

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
