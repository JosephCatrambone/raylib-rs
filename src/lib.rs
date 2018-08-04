#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;
use std::os::raw::*;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_init() {
		unsafe {
			let width: c_int = 640i32;
			let height: c_int = 480i32;
			let title:CString = CString::new("Oshit waddup").unwrap();
			//let title: *const ::std::os::raw::c_char = "Oshit waddup".to_string();
			InitWindow(width, height, title.as_ptr());
		}
	}

	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
