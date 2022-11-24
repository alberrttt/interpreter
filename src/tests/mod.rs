#[cfg(test)]
mod tests {
    use crate::frontend::{
        ast::{BinaryExpr, BinaryOperation, Expression, Literal, Node},
        parser,
        scanner::Scanner,
    };

    #[test]
    pub fn expression() {
        let mut scanner = Scanner::new(String::from("1+2"));
        scanner.scan_thru();
        let mut parser = parser::Parser::new(scanner.tokens.to_owned());
        let parsed = parser.parse();

        println!("{:#?}", parsed);

        let parsed = match parsed {
            Node::Expression(expr) => expr,
            _ => panic!(),
        };
        let parsed = match parsed {
            Expression::Binary(expr) => expr,
            _ => panic!(),
        };

        assert_eq!(parsed.op, BinaryOperation::Add);
    }
}
