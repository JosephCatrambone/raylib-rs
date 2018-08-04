#![allow(unused_imports, dead_code, unused_variables)]

extern crate bindgen;
extern crate cmake;

use std::path::{Path, PathBuf};
use std::{io, fs, env};

fn run_cmake(raylib_path: &Path) {
	let mut cfg = cmake::Config::new(raylib_path);
	// Force release config.  Does raylib have dev?
	cfg.profile("release");
	cfg.define("BUILD_EXAMPLES", "OFF");
	cfg.define("BUILD_GAMES", "OFF");
	//cfg.cflag(flag)
	let dst = cfg.build();
	//println!("cargo:rustc-link-search=native={}", dst.display());
	println!("cargo:rustc-link-search=native={}/lib", dst.display());
	println!("cargo:rustc-link-lib=static=raylib");
}

fn build_bindings() {
	let bindings = bindgen::Builder::default().header("raylib/src/raylib.h").generate().expect("Unable to generate bindings.");
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings.write_to_file(out_path.join("bindings.rs")).expect("Unable t- generate bindings file.");
}

fn main() {
	run_cmake(Path::new("raylib/"));
	build_bindings();
}