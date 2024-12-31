use emacs::{defun, Env, Result, Value};

emacs::plugin_is_GPL_compatible!();

#[emacs::module(name = "tokenizers")]
fn init(env: &Env) -> Result<Value<'_>> {
    env.message("Done Loading!")
}

#[defun]
fn say_hello(env: &Env, _name: String) -> Result<Value<'_>> {
    env.message("Hello")
}
