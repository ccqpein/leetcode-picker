use emacs::{defun, Env, IntoLisp, Result, Value};

// Emacs won't load the module without this.
emacs::plugin_is_GPL_compatible!();

// Register the initialization hook that Emacs will call when it loads the module.
#[emacs::module(name = "leetcode-picker-inner")]
fn init(env: &Env) -> Result<Value<'_>> {
    env.message("Done loading!")
}

// #[defun(mod_in_name = false)]
// fn create_md_buffer(env: &Env, buffer_name: String) -> Result<Value<'_>> {
//     //env.call("interactive", ["^p".into_lisp(env)?])?;
//     env.call("get-buffer-create", [buffer_name.as_str().into_lisp(env)?])?;
//     env.call("set-buffer", [buffer_name.into_lisp(env)?])?;
//     env.call("markdown-mode", [])
// }

// #[defun(mod_in_name = false)]
// fn insert_to_buffer(env: &Env, buffer_name: String, content: String) -> Result<Value<'_>> {
//     env.call("set-buffer", [buffer_name.into_lisp(env)?])?;
//     env.call("insert", [content.into_lisp(env)?])
// }

#[defun(mod_in_name = false)]
fn set_token(env: &Env, token: String) -> Result<()> {
    Ok(super::set_token(&Some(token)))
}

// #[defun(mod_in_name = false)]
// fn get_random(env: &Env) -> Result<()> {
//     //Ok(super::set_token(token))
// }
