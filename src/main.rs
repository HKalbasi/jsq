use clap::Parser;
use deno_core::{
    v8::{self, Function, Local, Object},
    JsRuntime, RuntimeOptions,
};
use stream_json::State;

mod stream_json;

#[derive(Debug, Parser)]
enum Command {
    #[command(visible_alias = "m")]
    Map { function: String },
    #[command(visible_alias = "r")]
    Reduce { acc: String, function: String },
    #[command(visible_alias = "s")]
    Scan { acc: String, function: String },
}

fn main() {
    let cmd = Command::parse();
    match cmd {
        Command::Map { function } => {
            let mut runtime = JsRuntime::new(RuntimeOptions::default());
            let mut handle = runtime.handle_scope();
            let json: Local<'_, Object> = JsRuntime::eval(&mut handle, "JSON").unwrap();
            let json_parse: Local<'_, Function> =
                JsRuntime::eval(&mut handle, "JSON.parse").unwrap();
            let json_stringify: Local<'_, Function> =
                JsRuntime::eval(&mut handle, "JSON.stringify").unwrap();
            let mut state = State::new();
            loop {
                let input = state.next();
                let input = v8::String::new(&mut handle, &input).unwrap();
                let input = json_parse
                    .call(&mut handle, json.into(), &[input.into()])
                    .unwrap();
                let transform: Local<'_, Function> = JsRuntime::eval(&mut handle, &function).unwrap();
                let result = transform.call(&mut handle, json.into(), &[input]).unwrap();
                let result = json_stringify
                    .call(&mut handle, json.into(), &[result])
                    .unwrap();
                let result = result.to_string(&mut handle).unwrap();
                let result = result.to_rust_string_lossy(&mut handle);
                println!("{result}");
            }
        }
        Command::Reduce { acc, function } => todo!(),
        Command::Scan { acc, function } => todo!(),
    }
}
