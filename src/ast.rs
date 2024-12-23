#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(i64),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Multiply,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug)]
pub enum Statement {
    Let {
        name: String,
        value: Expression,
    },
    Print {
        value: Expression,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    Loop {
        body: Vec<Statement>,
    },
    Save {
        filename: Expression,
    },
    Expression(Expression),
}

pub type Program = Vec<Statement>;
