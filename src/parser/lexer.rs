use crate::parser::splitter::Lexeme;

const built_in_functions: [Expression; 1] = [
    Expression::new(LSQLType::NONE, vec!(LSQLType::NONE)) //Well known
];

enum LSQLType {
    NONE
}

struct Expression {
    return_type: LSQLType,
    arguments: Vec<LSQLType>
}

impl Expression {
    pub const fn new(return_type: LSQLType, arguments: Vec<LSQLType>) -> Expression {
        Self {
            return_type,
            arguments
        }
    }
}

pub fn analyse(lexems: Vec<Lexeme>) {
    for i in lexems {

    }
}

