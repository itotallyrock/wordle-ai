use std::path::Path;
use std::{env, fs};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("word_list.rs");

    let word_list: Vec<_> = include_str!("./dictionary.txt")
        .lines()
        // .map(|s| s.as_bytes().iter().copied().collect::<Vec<_>>()).flatten()
        .collect();

    fs::write(
        &dest_path,
        format!(
            r"
pub const WORD_LIST_LEN: usize = {};
pub const WORD_LIST: [&'static str; WORD_LIST_LEN] = {:?};",
            word_list.len(),
            word_list.as_slice()
        ),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=dictionary.txt");
}
