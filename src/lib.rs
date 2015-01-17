#![allow(unstable)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

extern crate libc;

type http_cb = extern fn (*mut http_parser) -> libc::c_int;
type http_data_cb = extern fn (*mut http_parser, *const libc::c_char, libc::size_t) -> libc::c_int;

enum http_parser_type
{
    HTTP_REQUEST,
    HTTP_RESPONSE,
    HTTP_BOTH
}

#[repr(C)]
struct http_parser {
    /** PRIVATE **/

    // Ignoring all non-public fields (28 bytes)
    _private: [libc::uint32_t; 7],

    /** PUBLIC **/
    data: *mut libc::c_void,
}

#[repr(C)]
struct http_parser_settings {
    on_message_begin: http_cb,
    on_url: http_data_cb,
    on_status: http_data_cb,
    on_header_field: http_data_cb,
    on_header_value: http_data_cb,
    on_headers_complete: http_cb,
    on_body: http_data_cb,
    on_message_complete: http_cb,
}

extern "C" fn on_message_begin(parser: *mut http_parser) -> libc::c_int {
    0
}

extern "C" fn on_url(parser: *mut http_parser, at: *const libc::c_char, length: libc::size_t) -> libc::c_int {
    0
}

extern "C" fn on_status(parser: *mut http_parser, at: *const libc::c_char, length: libc::size_t) -> libc::c_int {
    0
}

extern "C" fn on_header_field(parser: *mut http_parser, at: *const libc::c_char, length: libc::size_t) -> libc::c_int {
    0
}

extern "C" fn on_header_value(parser: *mut http_parser, at: *const libc::c_char, length: libc::size_t) -> libc::c_int {
    0
}

extern "C" fn on_headers_complete(parser: *mut http_parser) -> libc::c_int {
    0
}

extern "C" fn on_body(parser: *mut http_parser, at: *const libc::c_char, length: libc::size_t) -> libc::c_int {
    0
}

extern "C" fn on_message_complete(parser: *mut http_parser) -> libc::c_int {
    0
}

#[link(name = "http_parser")]
extern {
    fn http_parser_init(
        parser: *mut http_parser,
        type_: libc::c_int) -> libc::c_void;

    fn http_parser_execute(
        parser: *mut http_parser,
        settings: *const http_parser_settings,
        data: *const libc::c_char,
        len: libc::size_t) -> libc::size_t;
}
