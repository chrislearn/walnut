
lazy_static! {
	static ref OPERATORS: Vec<(&'static str, &'static str, u16)> = vec![
        ("or", "OR", 1),
        ("and", "and", 2),
        ("eq", "=", 3),
        ("neq", "<>", 3),
        ("is", "IS", 3),
        ("nis", "IS NOT", 3),
        ("in", "IN", 3),
        ("nin", "NOT IN", 3),
        ("like", "LIKE", 3),
        ("nlike", "NOT LIKE", 3),
        ("ilike", "ILIKE", 3),
        ("nilike", "NOT ILIKE", 3),
        ("lt", "<", 3),
        ("gt", ">", 3),
        ("lte", "<=", 3),
        ("gte", ">="), 3,
    ];
}


pub enum Node {
    Ident(String),
    Literal(String),
    Bool(bool),
    String(String),
    Number(String),
    NULL,
    BinaryExp(String),
    LogicalExp(String),
    PeriodCode,
    DquoteCode,
    OparenCode,
    CparenCode,
}
enum ExpressionType {

}
struct Operator {
    name: String,
    sname: String,
    poriory: u16,
}
struct Expression {
    type: ExpressionType,
    operatior: Operator,
    left: Expression,
    right: Expression,
    has_error: bool,
}

pub fn parse(raw: String) -> Result<String, String> {
    if raw.is_empty() {
        return Err("empty filter");
    }

    let tree = globble_expr();
    if tree.type != Node::BinaryExp && tree.type != Node::LogicalExp {
        return Err("invalid expression");
    }
}

fn binary_precedence(value) -> bool {

}

fn create_binary_expr(operator, left, right) -> Expression {
    Expression {
        type: ExpressionType::LogicalExp,
        operator: operator,
        left: left,
        right: right,
        has_error: false,
    }
}

fn is_decimal_digit(ch) -> bool {
    ch == 45 || (ch >= 48 && ch <= 57)
}

fn is_ident_start(ch) -> bool {
    ch == 95 || ch == 44 ||
    (ch >= 65 && ch <= 90) ||
    (ch >= 97 && ch <= 122) ||
    (ch >= 48 && ch <= 57) ||
    (ch >= 128 && !struct_key_exists(operators, ch))
}