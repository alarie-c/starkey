use crate::frontend::{expr::Expr, parser::State, token::Token};

pub fn unexpected_token<'a>(
    _last: Option<&'a Expr>,
    _token: &'a Token,
    state: &'a State,
) -> (Option<Expr>, String) {
    match state {
        State::PreParamFunctionExpr => (
            None,
            String::from("'=' operator isn't supported in function parameters"),
        ),
        _ => (None, String::from("Unexpected token '='")),
    }
}
