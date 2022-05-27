//use crate::env::Env;
use crate::expr::Expr;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    pub name: String,
    pub val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("let", s)?;
        let (s, _) = utils::extract_whitespace1(s)?; // New!

        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let s = utils::tag("=", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, val) = Expr::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                val,
            },
        ))
    }
    //pub(crate) fn eval(&self, env: &mut Env) {
    //env.store_binding(self.name.clone(), self.val.eval());
    //}
}
