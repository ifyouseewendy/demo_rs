#[no_mangle]
pub unsafe extern "C" fn shopify_runtime_allocate(length: u32) -> *mut u8 {
	std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(
		length as usize,
		8,
	))
}
