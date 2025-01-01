use emacs::{defun, IntoLisp, Env, Value, Vector};
use tokenizers::tokenizer::{Tokenizer};
use anyhow::Result;

emacs::plugin_is_GPL_compatible!();

#[emacs::module(name = "tokenizers")]
fn init(env: &Env) -> Result<Value<'_>> {
    env.message("Done Loading!")
}

#[defun(user_ptr)]
fn from_pretrained(string: String) -> Result<Tokenizer> {
    let tk_res = Tokenizer::from_pretrained(string, None);

    match tk_res {
        Ok(tk) => Ok(tk),
        Err(_error) => panic!("hehe"),
    }
}

// Encode the given string using tokenizer tok and return
// token_ids, type_ids, and attention_mask
#[defun]
fn encode<'a>(env: &'a Env, tok: &mut Tokenizer, string: String) -> Result<Value<'a>> {
    let enc_res = tok.encode(string, false);

    match enc_res {
        Ok(enc) => {
            let token_ids = env.vector(
                &enc.get_ids()
                    .iter()
                    .map(|&id| id.into_lisp(env))
                    .collect::<Result<Vec<_>>>()?
                )?;

            let type_ids = env.vector(
                &enc.get_type_ids()
                    .iter()
                    .map(|&id| id.into_lisp(env))
                    .collect::<Result<Vec<_>>>()?
                )?;

            let attention_mask = env.vector(
                &enc.get_attention_mask()
                    .iter()
                    .map(|&id| id.into_lisp(env))
                    .collect::<Result<Vec<_>>>()?
                )?;

            let output_list = env.list((token_ids, type_ids, attention_mask))?;
            Ok(output_list)
        },
        Err(_error) => panic!("hehe"),
    }
}

// Encode given vector of strings like in `encode` and return a
// list of batched token_ids, type_ids, and attention_mask
#[defun]
fn encode_batch<'a>(env: &'a Env, tok: &mut Tokenizer, strings: Vector) -> Result<()> {
    // TODO
    Ok(())
}
