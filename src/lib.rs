#![feature(phase)]
#![feature(macro_rules)]
#![unstable]

extern crate collections;
extern crate libc;
#[phase(plugin,link)] extern crate log;

/// An analogue of `try!()` for systemd FFI calls.
///
/// The parameter should be a call to a systemd FFI fn with an i32 return
/// value. It is called, and if the return is negative then `sd_try!()`
/// interprets it as an error code and returns IoError from the enclosing fn.
/// Otherwise, the value of `sd_try!()` is the non-negative value returned by
/// the FFI call.
#[macro_export]
macro_rules! sd_try(
    ($e:expr) => ({
        let ret: i32;
        unsafe {
            ret = $e;
        }
        if ret < 0 {
            return Err(::std::io::IoError::from_errno(ret.abs() as uint, false));
        }
        ret
    })
)

/// Given an Option<&str>, either returns a pointer to a const char*, or a NULL
/// pointer if None.
#[macro_export]
macro_rules! char_or_null(
    ($e:expr) => (match $e {
        Some(p) => p.to_c_str().as_ptr(),
        None => ptr::null()
    })
)

/// Contains definitions for low-level bindings.
///
/// Most of this module is Rust versions of the systemd headers. The goal of
/// this crate is to make it unattractive to ever use the FFI directly, but
/// it's there if you need it.
///
/// Items in this module corresponding to systemd functions are well-documented
/// by the systemd man pages.
#[unstable]
pub mod ffi;

/// High-level interface to the systemd journal.
///
/// The main interface for writing to the journal is `fn log()`, and the main
/// interface for reading the journal is `struct Journal`.
#[experimental]
pub mod journal;

/// Similar to `log!()`, except it accepts a func argument rather than hard
/// coding `::log::log()`, and it doesn't filter on `log_enabled!()`.
#[macro_export]
macro_rules! log_with(
    ($func:expr, $lvl:expr, $($arg:tt)+) => ({
        static LOC: ::log::LogLocation = ::log::LogLocation {
            line: line!(),
            file: file!(),
            module_path: module_path!()
        };
        let lvl = $lvl;
        let func = $func;
        format_args!(|args| { func(lvl, &LOC, args) }, $($arg)+)
    })
)

#[macro_export]
macro_rules! sd_journal_log(
    ($lvl:expr, $($arg:tt)+) => (log_with!(::systemd::journal::log, $lvl, $($arg)+))
)

/// High-level interface to the systemd daemon module.
#[experimental]
pub mod daemon;
