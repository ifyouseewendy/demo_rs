use std::boxed::Box;
mod meta;

/// Core structure which is bound to our memory layout
/// https://github.com/Shopify/runtime-engine/wiki/Memory-Layout
#[repr(C)]
pub struct MySlice<T: 'static> {
	p: &'static T,
	len: u32,
}

#[repr(C)]
pub struct Input<'a> {
	v_int: i32,
	v_str: &'a MySlice<u8>,
	v_slice: &'a MySlice<i32>,
}

#[repr(C)]
pub struct User {
	age: i32,
	name: &'static MySlice<u8>,
}

#[repr(C)]
pub struct Output {
	v_int: i32,
	v_str: &'static MySlice<u8>,
	v_slice: &'static MySlice<i32>,
	v_struct: &'static User,
}

#[no_mangle]
pub extern "C" fn run<'a>(input: &'a Input) -> &'static Output {
	let v_int = 42;
	let v_str = wrap_string("hello world");
	let v_slice = wrap_slice(&vec![1, 2, 3]);

	let age = 18;
	let name = wrap_string("Di");
	let v_struct = wrap_value(User { age, name });
	wrap_value(Output {
		v_int,
		v_str,
		v_slice,
		v_struct,
	})
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

fn wrap_slice<T: Clone>(v: &[T]) -> &'static MySlice<T> {
	let len = v.len() as u32;
	let p: &'static [T] = Box::leak(v.to_vec().into_boxed_slice());

	wrap_value(MySlice { p: &p[0], len })
}

fn wrap_string(s: &str) -> &'static MySlice<u8> {
	let len = s.bytes().len() as u32;
	let chars = s.bytes().collect::<Vec<u8>>().into_boxed_slice();
	let c: &'static mut [u8] = Box::leak(chars);

	wrap_value(MySlice { p: &c[0], len })
}

fn wrap_value<T>(v: T) -> &'static T {
	Box::leak(Box::new(v))
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
