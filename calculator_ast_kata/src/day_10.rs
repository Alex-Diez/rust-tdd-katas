use std::iter::{Enumerate, Peekable};
use std::str::{Chars, FromStr};

type Cursor<'c> = Peekable<Enumerate<Chars<'c>>>;

#[derive(Debug, PartialEq)]
pub enum Ast {
    Num(f64),
    Op(char, Box<Ast>, Box<Ast>),
}

impl Ast {
    fn index(chars: &mut Cursor) -> usize {
        match chars.peek() {
            Some(&(index, _)) => index,
            None => 0,
        }
    }

    fn parse_num(chars: &mut Cursor) -> Result<Self, ParseAstError> {
        let mut num = String::new();
        let index = Self::index(chars.by_ref());
        while let Some(&(index, char)) = chars.peek() {
            match char {
                '+' | '*' | '/' => break,
                '-' if !num.is_empty() => break,
                '0'..='9' | '.' | '-' => {
                    chars.next();
                    num.push(char);
                }
                _ => return Err(ParseAstError(index)),
            }
        }
        num.parse().map(Ast::Num).map_err(|_| ParseAstError(index))
    }

    fn parse_high_priority_op(chars: &mut Cursor) -> Option<char> {
        match chars.peek() {
            Some(&(_, '*')) | Some(&(_, '/')) => chars.next().map(|(_, op)| op),
            _ => None,
        }
    }

    fn parse_term(chars: &mut Cursor) -> Result<Self, ParseAstError> {
        let mut left = Self::parse_num(chars.by_ref());
        while let Some(op) = Self::parse_high_priority_op(chars.by_ref()) {
            left = match (left, Self::parse_num(chars.by_ref())) {
                (Ok(left), Ok(right)) => Ok(Ast::Op(op, Box::new(left), Box::new(right))),
                (Ok(_), Err(right)) => Err(right),
                (Err(left), _) => Err(left),
            }
        }
        left
    }

    fn parse_low_priority_op(chars: &mut Cursor) -> Option<char> {
        match chars.peek() {
            Some(&(_, '+')) | Some(&(_, '-')) => chars.next().map(|(_, op)| op),
            _ => None,
        }
    }

    fn parse_expression(chars: &mut Cursor) -> Result<Self, ParseAstError> {
        let mut left = Self::parse_term(chars.by_ref());
        while let Some(op) = Self::parse_low_priority_op(chars.by_ref()) {
            left = match (left, Self::parse_term(chars.by_ref())) {
                (Ok(left), Ok(right)) => Ok(Ast::Op(op, Box::new(left), Box::new(right))),
                (Ok(_), Err(right)) => Err(right),
                (Err(left), _) => Err(left),
            }
        }
        left
    }
}

impl FromStr for Ast {
    type Err = ParseAstError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut chars = source.chars().enumerate().peekable();
        Self::parse_expression(chars.by_ref())
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseAstError(usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        assert_eq!("5".parse(), Ok(Ast::Num(5.0)))
    }

    #[test]
    fn error() {
        assert_eq!(Ast::from_str("abc"), Err(ParseAstError(0)))
    }

    #[test]
    fn negative_number() {
        assert_eq!("-7".parse(), Ok(Ast::Num(-7.0)))
    }

    #[test]
    fn addition() {
        assert_eq!(
            "4+9".parse(),
            Ok(Ast::Op(
                '+',
                Box::new(Ast::Num(4.0)),
                Box::new(Ast::Num(9.0))
            ))
        )
    }

    #[test]
    fn left_hand_side_error_low_priority_op() {
        assert_eq!(Ast::from_str("abc+5"), Err(ParseAstError(0)))
    }

    #[test]
    fn right_hand_side_error_low_priority_op() {
        assert_eq!(Ast::from_str("5+abc"), Err(ParseAstError(2)))
    }

    #[test]
    fn subtraction() {
        assert_eq!(
            "6-9".parse(),
            Ok(Ast::Op(
                '-',
                Box::new(Ast::Num(6.0)),
                Box::new(Ast::Num(9.0))
            ))
        )
    }

    #[test]
    fn multiplication() {
        assert_eq!(
            "5*6".parse(),
            Ok(Ast::Op(
                '*',
                Box::new(Ast::Num(5.0)),
                Box::new(Ast::Num(6.0))
            ))
        )
    }

    #[test]
    fn division() {
        assert_eq!(
            "8/2".parse(),
            Ok(Ast::Op(
                '/',
                Box::new(Ast::Num(8.0)),
                Box::new(Ast::Num(2.0))
            ))
        )
    }

    #[test]
    fn many_operation() {
        assert_eq!(
            "5+4*2-27/3".parse(),
            Ok(Ast::Op(
                '-',
                Box::new(Ast::Op(
                    '+',
                    Box::new(Ast::Num(5.0)),
                    Box::new(Ast::Op(
                        '*',
                        Box::new(Ast::Num(4.0)),
                        Box::new(Ast::Num(2.0))
                    ))
                )),
                Box::new(Ast::Op(
                    '/',
                    Box::new(Ast::Num(27.0)),
                    Box::new(Ast::Num(3.0))
                ))
            ))
        )
    }

    #[test]
    fn unknown_operator() {
        assert_eq!(Ast::from_str("5&6"), Err(ParseAstError(1)))
    }
}
