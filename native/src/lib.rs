#[macro_use]
extern crate neon;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

register_module!(mut cx, { cx.export_function("hello", hello) });

#[cfg(test)]
mod tests {
    #[test]
    fn test_failure() {
        assert_eq!(1, 0);
    }
}
