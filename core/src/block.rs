use crate::stmt::Stmt;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let mut s = s;
        let mut stmts = Vec::new();

        while let Ok((new_s, stmt)) = Stmt::new(s) {
            s = new_s;
            stmts.push(stmt);

            let (new_s, _) = utils::extract_whitespace(s);
            s = new_s;
        }

        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("}", s)?;

        Ok((s, Block { stmts }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding_def::BindingDef;
    use crate::expr::Expr;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Stmt::new("let a = 10"),
            Ok((
                "",
                Stmt::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::Number(Number(10)),
                }),
            )),
        );
    }

    #[test]
    fn parse_expr() {
        assert_eq!(
            Stmt::new("1+1"),
            Ok((
                "",
                Stmt::Expr(Expr::Operation {
                    lhs: Number(1),
                    rhs: Number(1),
                    op: Op::Add,
                }),
            )),
        );
    }
    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { stmts: Vec::new() })));
    }
    #[test]
    fn parse_block_with_one_stmt() {
        assert_eq!(
            Block::new("{ 5 }"),
            Ok((
                "",
                Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(5)))],
                },
            )),
        );
    }
}
