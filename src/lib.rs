use emacs::{defun, IntoLisp, Env, Value, Vector};
use tokenizers::tokenizer::{Tokenizer};
use anyhow::Result;

emacs::plugin_is_GPL_compatible!();

#[emacs::module(name = "tokenizers")]
fn init(env: &Env) -> Result<Value<'_>> {
    env.message("Loaded tokenizers!")
}

#[defun(user_ptr)]
fn from_pretrained(string: String) -> Result<Tokenizer> {
    let tk_res = Tokenizer::from_pretrained(string, None);

    match tk_res {
        Ok(tk) => Ok(tk),
        Err(_error) => panic!("Error in loading tokenizer from pretrained"),
    }
}

// Enable padding to the longest sentence in the batch
#[defun]
fn enable_padding(tok: &mut Tokenizer, pad_id: u32, pad_token: String) -> Result<()> {
    tok.with_padding(Some(tokenizers::PaddingParams {
        pad_id, pad_token,
        ..tokenizers::PaddingParams::default()
    }));

    Ok(())
}

// Encode the given string using tokenizer tok and return
// token_ids, type_ids, and attention_mask
#[defun]
fn encode<'a>(env: &'a Env, tok: &mut Tokenizer, string: String, add_special_tokens: Value) -> Result<Value<'a>> {
    let enc_res = tok.encode(string, add_special_tokens.eq(env.intern("t")?));

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
        Err(_error) => panic!("Error in encoding"),
    }
}

// Encode given vector of strings like in `encode` and return a
// list of batched token_ids, type_ids, and attention_mask
#[defun]
fn encode_batch<'a>(env: &'a Env, tok: &mut Tokenizer, strings: Vector, add_special_tokens: Value) -> Result<Value<'a>> {
    // Emacs vector to rust vector
    let n_strings: usize = env.call("length", (strings,))?.into_rust()?;
    let mut strings_v = vec![String::new(); n_strings];

    for i in 0..n_strings {
        strings_v[i] = env.call("aref", (strings, i))?.into_rust()?;
    }

    let enc_res = match tok.encode_batch(strings_v, add_special_tokens.eq(env.intern("t")?)) {
        Ok(res) => res,
        Err(_error) => panic!("Error in batch encoding"),
    };

    let mut token_ids_2d: Vec<Value> = Vec::with_capacity(n_strings);
    let mut type_ids_2d: Vec<Value> = Vec::with_capacity(n_strings);
    let mut attention_mask_2d: Vec<Value> = Vec::with_capacity(n_strings);

    for enc in &enc_res {
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

        token_ids_2d.push(token_ids);
        type_ids_2d.push(type_ids);
        attention_mask_2d.push(attention_mask);
    }

    let output_list = env.list((env.vector(&token_ids_2d)?,
                                env.vector(&type_ids_2d)?,
                                env.vector(&attention_mask_2d)?))?;

    Ok(output_list)
}
