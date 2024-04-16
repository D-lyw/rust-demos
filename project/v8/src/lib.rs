mod state;

use v8::{new_default_platform, Isolate, OwnedIsolate, V8};

pub struct JsRuntime {
    isolate: OwnedIsolate,
}

pub struct JsRuntimeParams {}

impl JsRuntime {
    pub fn init() {
        let platfrom = new_default_platform(0, false).make_shared();
        V8::initialize_platform(platfrom);
        V8::initialize();
    }

    pub fn new(params: JsRuntimeParams) -> Self {
        let isolate = Isolate::new(Default::default());

        JsRuntime { isolate: isolate }
    }

    pub fn exexute_script(&mut self, script: impl AsRef<str>) {
        todo!()
        let context = JsRuntimeState::new(&mut self.isolate).get_context();
    }

    pub fn create_snapshot() {
        todo!()
    }
}
