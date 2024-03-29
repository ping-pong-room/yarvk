#![feature(get_mut_unchecked)]

use crate::generate_device_features::generate_device_features;
use crate::generate_extensions::generate_extensions;
use crate::variadic_generic::descriptor_set_variadic_generics;
use quote::__private::TokenStream;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

mod generate_device_features;
mod generate_extensions;
mod variadic_generic;

fn write_fs_and_format<P: AsRef<Path> + ?Sized>(path: &P, data: TokenStream) {
    let mut file = File::create(path).unwrap();
    write!(file, "{data}").unwrap();
    Command::new("rustfmt")
        .arg(path.as_ref().as_os_str())
        .spawn()
        .unwrap();
}

fn main() {
    let vk_xml = PathBuf::new().join("generator").join("vk.xml");
    let (spec2, _errors) = vk_parse::parse_file(&vk_xml).expect("Invalid xml file");

    let (res, ext_infos) = generate_extensions(&spec2);
    write_fs_and_format("yarvk/src/extensions.rs", res);

    let res = generate_device_features(&spec2, &ext_infos);
    write_fs_and_format("yarvk/src/device_features.rs", res);

    let (public_res, private_res) = descriptor_set_variadic_generics(2);
    write_fs_and_format(
        "yarvk/src/descriptor_set/descriptor_variadic_generics.rs",
        public_res,
    );
    write_fs_and_format(
        "yarvk/src/descriptor_set/private_descriptor_variadic_generics.rs",
        private_res,
    );
}
