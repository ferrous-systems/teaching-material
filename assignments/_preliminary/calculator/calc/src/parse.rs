use crate::Expr;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedInput(String),
    InsufficientNumbers,
    EmptyStack,
    ExcessStack,
}

/// Parse a postfix notation string expression to a rendered AST
///
/// This function takes input in the form "4 3 +", and will produce
/// an `Expr` item.
///
/// ## Examples
///
/// ```rust
/// # use calc::parse::parse;
/// #
/// let text = "4 3 +";
/// let ast_expr = parse(text).unwrap();
/// ```
///
/// ```rust
/// # use calc::parse::parse;
/// #
/// let text = "3 sqr";
/// let ast_expr = parse(text).unwrap();
/// ```
pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Expr> = Vec::new();

    for word in input.split_ascii_whitespace() {
        match word {
            "sqr" => {
                let item = match stack.pop() {
                    Some(i) => i,
                    None => return Err(ParseError::InsufficientNumbers),
                };

                let exp_sqr = Expr::Sqr(Box::new(item));
                stack.push(exp_sqr);
            }
            "+" | "-" | "*" | "/" => {
                let (a, b) = match (stack.pop(), stack.pop()) {
                    (Some(b), Some(a)) => (a, b),
                    _ => return Err(ParseError::InsufficientNumbers),
                };

                let expr = match word {
                    "+" => Expr::Plus(Box::new(a), Box::new(b)),
                    "-" => Expr::Minus(Box::new(a), Box::new(b)),
                    "*" => Expr::Multiply(Box::new(a), Box::new(b)),
                    "/" => Expr::Divide(Box::new(a), Box::new(b)),
                    whatever => {
                        // Shouldn't be possible, checked
                        return Err(ParseError::UnexpectedInput(whatever.to_string()));
                    }
                };

                stack.push(expr);
            }
            x => {
                if let Ok(num) = x.parse::<i64>() {
                    let exp_num = Expr::Number(num);
                    stack.push(exp_num);
                } else {
                    let owned = x.to_string();
                    let err = ParseError::UnexpectedInput(owned);
                    return Err(err);
                }
            }
        }
    }

    // We hit this case if the input is empty!
    if stack.is_empty() {
        return Err(ParseError::EmptyStack);
    } else if stack.len() > 1 {
        return Err(ParseError::ExcessStack);
    }

    // We know stack.len() == 1, unwrap okay!
    let res = stack.pop().unwrap();
    Ok(res)
}
