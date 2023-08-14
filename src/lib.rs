use neon::prelude::*;
use std::time::Duration;

fn sleep_sync(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let millis = cx.argument::<JsNumber>(0)?;
    let timeout = Duration::from_secs_f64(millis.value(&mut cx) * 1e-3);

    std::thread::sleep(timeout);
    return Ok(cx.undefined());
}

fn yield_sync(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    std::thread::yield_now();
    return Ok(cx.undefined());
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("sleepSync", sleep_sync)?;
    cx.export_function("yieldSync", yield_sync)?;
    Ok(())
}
