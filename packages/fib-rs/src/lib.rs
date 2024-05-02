// #[no_mangle]
// pub extern "C" fn fib(n: i32) -> i32 {
//     return match n {
//         1 | 2 => 1,
//         n => fib(n - 1) + fib(n - 2),
//     };
// }

/// import the preludes
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// module registration is done by the runtime, no need to explicitly do it now.
#[napi]
fn fib(n: u32) -> u32 {
    match n {
        1 | 2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
