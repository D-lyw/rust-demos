
use v8::{new_default_platform, Context, ContextScope, HandleScope, Isolate, Script, String, V8};

fn main() {
    init();

    let isolate = &mut Isolate::new(Default::default());

    let scope = &mut HandleScope::new(isolate);
    let context = Context::new(scope);
    let scope = &mut ContextScope::new(scope, context);

    let code = String::new(scope, "'hello' + 'world'").unwrap();
    println!("Source code: {}", code.to_rust_string_lossy(scope));
    let script = Script::compile(scope, code, None).unwrap();

    let result = script.run(scope).unwrap();
    println!("result: {:?}", result.to_rust_string_lossy(scope));
}

fn init() {
    let platfrom = new_default_platform(0, false).make_shared();
    V8::initialize_platform(platfrom);
    V8::initialize();

}