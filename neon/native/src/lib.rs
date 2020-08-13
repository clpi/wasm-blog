#[macro_use]
extern crate neon;

use neon::prelude::*;
use rayon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let now = std::time::Instant::now();
    let mut num: i32 = 0;
    for i in 1..10000 { num += i; }
    let after = std::time::Instant::now();
    let dur = after.duration_since(now).as_secs_f64();
    Ok(cx.string(format!("{} seconds: Hello, from rust! {}", dur, num)))
}

fn hello_par(mut cx: FunctionContext) -> JsResult<JsString> {
    let now = std::time::Instant::now();
    let num: usize = (1..10000usize).into_par_iter().sum();
    let after = std::time::Instant::now();
    let dur = after.duration_since(now).as_secs_f64();
    Ok(cx.string(format!("{} seconds: Hello, from rust (parallel)! {}", dur, num)))
}

fn test(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("D".to_string()))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("hello_par", hello_par)?;
    Ok(())
});
