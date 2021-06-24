use super::{Level, Quiz};
use emacs::{defun, Env, Error, IntoLisp, Result, Value};

// Emacs won't load the module without this.
emacs::plugin_is_GPL_compatible!();

// Register the initialization hook that Emacs will call when it loads the module.
#[emacs::module(name = "leetcode-picker-inner")]
fn init(env: &Env) -> Result<Value<'_>> {
    env.message("Done loading!")
}

#[defun(mod_in_name = false)]
fn set_token(env: &Env, token: String) -> Result<()> {
    Ok(super::set_token(&Some(token)))
}

#[defun(mod_in_name = false)]
fn get_random(env: &Env, format_str: Option<String>) -> Result<Value<'_>> {
    let qq = Quiz::get_randomly(None).unwrap();

    qq.use_fmt_temp(&format_str, &None).unwrap().into_lisp(env)
}

#[defun(mod_in_name = false)]
fn get_random_with_level(env: &Env, level: Level, format_str: Option<String>) -> Result<Value<'_>> {
    let qq = Quiz::get_randomly(Some(level)).unwrap();

    qq.use_fmt_temp(&format_str, &None).unwrap().into_lisp(env)
}
