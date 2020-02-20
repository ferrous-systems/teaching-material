use std::str::FromStr;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Square(Box<Expr>),
    BinOp {
        op_kind: BinOpKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum BinOpKind {
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEof,
    UnexpectedToken,
    ExtraInput,
}

#[derive(Debug)]
pub enum EvalError {
    DivisionByZero,
}

impl FromStr for Expr {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Expr, ParseError> {
        let mut tokens = s.split_ascii_whitespace();
        let expr = parse_expr(&mut tokens)?;
        if tokens.next().is_some() {
            return Err(ParseError::ExtraInput);
        }
        Ok(expr)
    }
}

fn parse_expr<'a, I: Iterator<Item = &'a str>>(tokens: &mut I) -> Result<Expr, ParseError> {
    let first = tokens.next().ok_or(ParseError::UnexpectedEof)?;
    let op_kind = match first {
        "+" => BinOpKind::Add,
        "-" => BinOpKind::Sub,
        "/" => BinOpKind::Div,
        "*" => BinOpKind::Mul,
        "sqr" => {
            let operand = parse_expr(tokens)?;
            return Ok(Expr::Square(Box::new(operand)));
        }
        _ => {
            let number = first
                .parse::<i64>()
                .map_err(|_| ParseError::UnexpectedToken)?;
            return Ok(Expr::Number(number));
        }
    };
    let lhs = parse_expr(tokens)?;
    let rhs = parse_expr(tokens)?;
    Ok(Expr::BinOp {
        op_kind,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    })
}

impl Expr {
    pub fn eval(&self) -> Result<i64, EvalError> {
        let res = match self {
            Expr::Number(it) => *it,
            Expr::BinOp { op_kind, lhs, rhs } => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                match op_kind {
                    BinOpKind::Add => lhs + rhs,
                    BinOpKind::Sub => lhs - rhs,
                    BinOpKind::Div => lhs.checked_div(rhs).ok_or(EvalError::DivisionByZero)?,
                    BinOpKind::Mul => lhs * rhs,
                }
            }
            Expr::Square(operand) => {
                let operand = operand.eval()?;
                operand * operand
            }
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_happy(input: &str, expected: i64) {
        let expr: Expr = input.parse().unwrap();
        let value = expr.eval().unwrap();
        assert_eq!(value, expected)
    }

    #[test]
    fn it_works() {
        check_happy("92", 92);
        check_happy("+ 40 2", 42);
        check_happy("/ + 1 3 2", 2);
        check_happy("- + sqr 3 sqr 4 sqr 5", 0);
    }
}
