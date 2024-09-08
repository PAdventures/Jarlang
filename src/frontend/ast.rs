#[derive(Debug, Clone)]
pub enum NodeType {
    // Statements
    ProgramStatement,
    VariableDeclarationStatement,

    // Other
    Identifier,

    // Literal data types
    IntegerLiteral,
    FloatLiteral,
    CharacterLiteral,
    StringLiteral,

    // Expressions
    BinaryExpression,
    VariableAssignmentExpression,
}

// Statements

#[derive(Debug)]
pub enum Statement {
    Program(ProgramStatement),
    VariableDeclaration(VariableDeclarationStatement),
    Expression(Expression),
}

#[derive(Debug)]
pub struct ProgramStatement {
    pub kind: NodeType,
    pub body: Vec<Statement>,
}

impl ProgramStatement {
    pub fn create() -> Self {
        ProgramStatement {
            kind: NodeType::ProgramStatement,
            body: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableDeclarationStatement {
    pub kind: NodeType,
    pub constant: bool,
    pub identifier: String,
    pub value: Option<Expression>,
    pub value_type: Option<IdentifierExpression>,
}

impl VariableDeclarationStatement {
    pub fn create(
        constant: bool,
        identifier: String,
        value: Option<Expression>,
        value_type: Option<IdentifierExpression>,
    ) -> Self {
        VariableDeclarationStatement {
            kind: NodeType::VariableDeclarationStatement,
            constant,
            identifier,
            value,
            value_type,
        }
    }
}

// Expressions

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(Box<BinaryExpression>),
    VariableAssignment(Box<VariableAssignmentExpression>),
    Identifier(IdentifierExpression),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Character(CharacterLiteral),
    String(StringLiteral),
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub kind: NodeType,
    pub left: Expression,
    pub right: Expression,
    pub operator: String,
}

impl BinaryExpression {
    pub fn create(left: Expression, right: Expression, operator: String) -> Self {
        BinaryExpression {
            kind: NodeType::BinaryExpression,
            left,
            right,
            operator,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableAssignmentExpression {
    pub kind: NodeType,
    pub assignee: Expression,
    pub value: Expression,
}

impl VariableAssignmentExpression {
    pub fn create(assignee: Expression, value: Expression) -> Self {
        VariableAssignmentExpression {
            kind: NodeType::VariableAssignmentExpression,
            assignee,
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentifierExpression {
    pub kind: NodeType,
    pub symbol: String,
}

impl IdentifierExpression {
    pub fn create(symbol: String) -> Self {
        IdentifierExpression {
            kind: NodeType::Identifier,
            symbol,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub kind: NodeType,
    pub value: String,
}

impl IntegerLiteral {
    pub fn create(value: String) -> Self {
        IntegerLiteral {
            kind: NodeType::IntegerLiteral,
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FloatLiteral {
    pub kind: NodeType,
    pub value: String,
}

impl FloatLiteral {
    pub fn create(value: String) -> Self {
        FloatLiteral {
            kind: NodeType::FloatLiteral,
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharacterLiteral {
    pub kind: NodeType,
    pub value: char,
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub kind: NodeType,
    pub value: String,
}
