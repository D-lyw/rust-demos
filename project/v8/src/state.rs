use std::{cell::RefCell, rc::Rc};

use v8::{Context, Global, HandleScope, Isolate};


pub struct JsRuntimeState {
    context: Option<Global<Context>>
}

impl JsRuntimeState {
    pub fn new(isolate: &mut Isolate) -> Rc<RefCell<JsRuntimeState>> {

        let context = {
            let scope = &mut HandleScope::new(isolate);
            let context = Context::new(scope);
            Global::new(scope, context)
        };
        
        Rc::new(RefCell::new(JsRuntimeState {
            context: Some(context)
        }))
    }

    pub fn get_context(&self) -> Global<Context> {
        self.context.as_ref().unwrap().clone()
    }

}