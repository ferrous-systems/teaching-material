use std::str::FromStr;

// TODO Step 2: Define the Abstract Syntax Tree (AST) structure for postfix expressions.
#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Square(Box</* TODO: recursive type! */>),
    BinOp {
        op_kind: BinOpKind,
        lhs: Box</* TODO: recursive type! */>,
        rhs: Box</* TODO: recursive type! */>,
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

// TODO Step 4 & 5: implement function for parsing a string into an `Expr`
fn parse(input: &str) -> Result<Expr, ParseError> {
    // Create a variable to hold the stack of expressions
    let mut stack: Vec<Expr> = Vec::new();
    for word in input.split_ascii_whitespace() {
        // TODO: For each “word” of the input string, classify it as either one of the operators
        //       or as a number (match will be useful here).

        // TODO: If the word is a number, push an expression representing the number to the stack
        //       (use the parse function to convert strings into integers).

        // TODO: If the word is an operator, pop one (for sqr) or two (for +, -, *, /) operands from
        //       the stack, use them to create a compound Expr, and push the result back onto the stack.

        // TODO: Don’t forget to handle error conditions (unexpected token, pop from an empty stack,
        //       more than one value on the stack after the end of input)

    };
    assert_eq!(stack.len(), 1);
    let res = stack.pop().unwrap();
    Ok(res)
}


impl Expr {
    // TODO Step 3: Define a recursive eval function to compute the value of an expression.
    pub fn eval(&self) -> Result<i64, EvalError> {
        let res = match self {
            Expr::Number(it) => /* TODO: return the value contained in `Number()` here */,
            Expr::BinOp { op_kind, lhs, rhs } => {
                // TODO: evaluate left hand side (`lhs`) and right hand side (`rhs`)

                // TODO: depending on `op_kind`, perform operation involving `rhs` and `lhs`
                //       👀 take a peek at `Expr::Square(operand)` for reference
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
