use std::boxed::Box;
mod meta;

/// Core structure which is bound to our memory layout
#[repr(C)]
pub struct MySlice<T: 'static> {
	p: &'static T,
	len: u32,
}

impl<T: Clone> MySlice<T> {
	fn to_vec(&self) -> Vec<T> {
		let raw_p = self.p as *const _;
		let len = self.len as usize;
		let slice = unsafe { std::slice::from_raw_parts(raw_p, len) };
		slice.to_vec()
	}
}

impl MySlice<u8> {
	fn to_string(&self) -> String {
		String::from_utf8_lossy(&self.to_vec()).to_string()
	}
}

#[repr(C)]
pub struct Input<'a> {
	v_int: i32,
	v_str: &'a MySlice<u8>,
	v_slice: &'a MySlice<i32>,
	v_struct: &'static User,
	v_slice_of_struct: &'static MySlice<&'static User>,
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
	v_slice_of_struct: &'static MySlice<&'static User>,
}

#[no_mangle]
pub extern "C" fn run<'a>(input: &'a Input) -> &'static Output {
	let v_int = input.v_int;
	let v_str = wrap_string(&input.v_str.to_string());
	let v_slice = wrap_slice(&input.v_slice.to_vec());

	let v_struct = input.v_struct;
	let v_struct = wrap_value(v_struct);
	let v_slice_of_struct = wrap_slice(&input.v_slice_of_struct.to_vec());
	wrap_value(Output {
		v_int,
		v_str,
		v_slice,
		v_struct,
		v_slice_of_struct,
	})
}

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
