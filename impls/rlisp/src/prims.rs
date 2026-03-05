use crate::types::*;

pub fn builtin_add(l: Expr, r: Expr) -> MalResult<Expr> {
    let li = l.as_i64()?;
    let lr = r.as_i64()?;
    Ok(Expr::number(li + lr))
}

pub fn builtin_sub(l: Expr, r: Expr) -> MalResult<Expr> {
    let li = l.as_i64()?;
    let lr = r.as_i64()?;
    Ok(Expr::number(li - lr))
}

pub fn builtin_div(l: Expr, r: Expr) -> MalResult<Expr> {
    let li = l.as_i64()?;
    let lr = r.as_i64()?;
    Ok(Expr::number(li / lr))
}

pub fn builtin_mul(l: Expr, r: Expr) -> MalResult<Expr> {
    let li = l.as_i64()?;
    let lr = r.as_i64()?;
    Ok(Expr::number(li * lr))
}
