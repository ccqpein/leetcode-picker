use super::{Level, Quiz};
use emacs::{defun, Env, Error, IntoLisp, Result, Value};
use lazy_static::*;
use std::sync::Mutex;

// Emacs won't load the module without this.
emacs::plugin_is_GPL_compatible!();

lazy_static! {
    static ref CACHE_QUiZ: Mutex<Option<Quiz>> = Mutex::new(None);
}

// Register the initialization hook that Emacs will call when it loads the module.
#[emacs::module(name = "leetcode-picker-inner")]
fn init(env: &Env) -> Result<Value<'_>> {
    env.message("Done loading!")
}

#[defun(mod_in_name = false)]
fn set_token(env: &Env, token: String) -> Result<()> {
    Ok(super::set_token(&Some(token)))
}

/// get random quiz and return description
/// update two static var at same time
#[defun(mod_in_name = false)]
fn get_random(env: &Env, format_str: Option<String>) -> Result<Value<'_>> {
    let qq = Quiz::get_randomly(None).unwrap();

    // update
    update_static_quiz(&qq);

    qq.use_fmt_temp(&format_str, &None).unwrap().into_lisp(env)
}

#[defun(mod_in_name = false)]
fn get_random_with_level(env: &Env, level: Level, format_str: Option<String>) -> Result<Value<'_>> {
    let qq = Quiz::get_randomly(Some(level)).unwrap();

    update_static_quiz(&qq);

    qq.use_fmt_temp(&format_str, &None).unwrap().into_lisp(env)
}

/// get the last quiz snippet
#[defun(mod_in_name = false)]
fn get_code_snippet(env: &Env, lang: String) -> Result<String> {
    match CACHE_QUiZ.lock() {
        Ok(ref mut q) => Ok(q.as_ref().unwrap().code_snippet(&lang).unwrap().to_string()),
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn update_static_quiz(qq: &Quiz) {
    // update quiz
    match CACHE_QUiZ.lock() {
        Ok(ref mut q) => {
            **q = Some(qq.clone());
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
