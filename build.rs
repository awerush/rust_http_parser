#![allow(unstable)]

extern crate gcc;

fn main() {
    let flags = vec![
        "-I.".to_string(),
        "-DHTTP_PARSER_STRICT=0".to_string(),
        "-Wall".to_string(),
        "-Wextra".to_string(),
        "-O3".to_string(),
    ];

    let config = gcc::Config {
        include_directories: vec![],
        definitions: vec![],
        objects: vec![],
        flags: flags };

    gcc::compile_library(
        "libhttp_parser.a",
        &config,
        &["http-parser/http_parser.c"]);
}
