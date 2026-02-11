use kf_compiler::parser::parser::{Program, Statement, StatementKind, Type};

pub struct Symbol {
    pub name: String,          
    pub symbol_type: SymbolType, 
    pub data_type: String,       
    pub value: Option<String>,   
    pub scope: String,           
    pub line: usize,            
    pub col: usize,            
}

pub enum SymbolType {
    Variable,
    Constant,
    Function,
    Parameter,
}

pub fn extract_symbols(program: &Program) -> Vec<Symbol>{
    let mut symbols = Vec::new();

    for function in &program.functions {
        extract_symbols_from_statements(&function.body, &function.name, &mut symbols);
    }
    
    symbols
}

fn extract_symbols_from_statements(statements: &[Statement], scope: &str, symbols: &mut Vec<Symbol>) {
    for stmt in statements {
        match &stmt.kind {
            StatementKind::VariableDeclaration { name, var_type, value } => {
                symbols.push(Symbol {
                    name: name.clone(),
                    symbol_type: SymbolType::Variable,
                    data_type: type_to_string(var_type),
                    value: Some(expression_to_string(value)),
                    scope: scope.to_string(),
                    line: stmt.span.start.line,
                    col: stmt.span.start.col,
                });
            }
            
            StatementKind::ConstDeclaration { name, var_type, value } => {
                symbols.push(Symbol {
                    name: name.clone(),
                    symbol_type: SymbolType::Constant,
                    data_type: type_to_string(var_type),  
                    value: Some(expression_to_string(value)),  
                    scope: scope.to_string(),  
                    line: stmt.span.start.line,  
                    col: stmt.span.start.col,  
                });
            }
            
            StatementKind::If { then_block, elif_blocks, else_block, .. } => {
                extract_symbols_from_statements(then_block, scope, symbols);
                for (_, block) in elif_blocks {
                    extract_symbols_from_statements(block, scope, symbols);
                }
                if let Some(else_stmts) = else_block {
                    extract_symbols_from_statements(else_stmts, scope, symbols);
                }
            }
            
            StatementKind::Switch { cases, .. } => {
                for case in cases {
                    extract_symbols_from_statements(&case.body, scope, symbols);
                }
            }

            StatementKind::For { init, body, .. } => {
                // Extraer la variable del init del for
                if let StatementKind::VariableDeclaration { name, var_type, value } = &init.kind {
                    symbols.push(Symbol {
                        name: name.clone(),
                        symbol_type: SymbolType::Variable,
                        data_type: type_to_string(var_type),
                        value: Some(expression_to_string(value)),
                        scope: scope.to_string(),
                        line: init.span.start.line,
                        col: init.span.start.col,
                    });
                }
 
                extract_symbols_from_statements(body, scope, symbols);
            }
            
            _ => {}
        }
    }
}


fn type_to_string(t: &Type) -> String {
    match t {
        Type::Ont => "ont".to_string(),
        Type::Uont => "uont".to_string(),
        Type::Michi => "michi".to_string(),
        Type::Ntr => "ntr".to_string(),
        Type::Chip => "chip".to_string(),
        Type::Yesorno => "yesorno".to_string(),
    }
}

fn expression_to_string(expr: &kf_compiler::parser::parser::Expression) -> String {
    use kf_compiler::parser::parser::ExpressionKind;
    
    match &expr.kind {
        ExpressionKind::IntegerLiteral(n) => n.to_string(),
        ExpressionKind::FloatLiteral(f) => f.to_string(),
        ExpressionKind::CharLiteral(c) => format!("'{}'", c),
        ExpressionKind::StringLiteral(s) => format!("\"{}\"", s),
        ExpressionKind::BoolLiteral(b) => if *b { "yes" } else { "no" }.to_string(),
        ExpressionKind::Identifier(id) => id.clone(),
        
        ExpressionKind::BinaryOp { op, left, right } => {
            let op_str = match op {
                kf_compiler::parser::parser::BinaryOperator::Add => "plus",
                kf_compiler::parser::parser::BinaryOperator::Subtract => "minus",
                kf_compiler::parser::parser::BinaryOperator::Multiply => "mult",
                kf_compiler::parser::parser::BinaryOperator::Divide => "by",
                kf_compiler::parser::parser::BinaryOperator::Modulo => "mod",
                kf_compiler::parser::parser::BinaryOperator::Equal => "eq",
                kf_compiler::parser::parser::BinaryOperator::NotEqual => "noteq",
                kf_compiler::parser::parser::BinaryOperator::GreaterThan => "great",
                kf_compiler::parser::parser::BinaryOperator::LessThan => "lesst",
                kf_compiler::parser::parser::BinaryOperator::And => "and",
                kf_compiler::parser::parser::BinaryOperator::Or => "or",
                kf_compiler::parser::parser::BinaryOperator::Join => "join",
            };
            format!("{} {} {}", expression_to_string(left), op_str, expression_to_string(right))
        }
        
        ExpressionKind::UnaryOp { op, operand } => {
            let op_str = match op {
                kf_compiler::parser::parser::UnaryOperator::Not => "nah",
                kf_compiler::parser::parser::UnaryOperator::Negate => "minus",
            };
            format!("{} {}", op_str, expression_to_string(operand))
        }
        
        ExpressionKind::FunctionCall { name, args } => {
            let args_str = args.iter()
                .map(expression_to_string)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({})", name, args_str)
        }
        
        ExpressionKind::ArrayAccess { array, index } => {
            format!("{}[{}]", array, expression_to_string(index))
        }
        
        ExpressionKind::PostfixOp { op, operand } => {
            let op_str = match op {
                kf_compiler::parser::parser::PostfixOperator::Increment => "plusplus",
                kf_compiler::parser::parser::PostfixOperator::Decrement => "minusminus",
            };
            format!("{}{}", expression_to_string(operand), op_str)
        }
    }
}