use crate::ast::ASTNode;
use crate::ast::Expression;
use crate::ast::Operator;
use crate::ast::Value;

pub struct Transpiler {
    nodes: Vec<ASTNode>,
}

impl Transpiler {
    pub fn new(nodes: Vec<ASTNode>) -> Self {
        Self { nodes }
    }

    pub fn transpile(&self) -> String {
        let mut code = String::new();

        for node in &self.nodes {
            code.push_str(Transpiler::transpile_node(node).as_str());
        }

        code
    }

    fn transpile_node(node: &ASTNode) -> String {
        let mut code = String::new();

        match node {
            ASTNode::FunctionDeclaration {
                start: _,
                name,
                parameters,
                content,
            } => {
                let params = parameters.join(", ");
                let body = content
                    .iter()
                    .map(Transpiler::transpile_node)
                    .collect::<Vec<_>>()
                    .join("\n");

                code.push_str(format!("function {name}({params}) {{\n{body}\n}}\n").as_str());
            }
            ASTNode::VariableDeclaration { start: _, vars } => {
                let vars = vars
                    .iter()
                    .map(|v| format!("{}={}", v.0, Transpiler::transpile_expression(&v.1)))
                    .collect::<Vec<_>>()
                    .join(",");

                code.push_str(format!("let {vars};").as_str());
            }
            ASTNode::Expression(expression) => {
                code.push_str(
                    format!("{};", Transpiler::transpile_expression(expression)).as_str(),
                );
            }
            ASTNode::VariableSetting { start: _, vars } => {
                let vars = vars
                    .iter()
                    .map(|v| format!("{}={}", v.0, Transpiler::transpile_expression(&v.1)))
                    .collect::<Vec<_>>()
                    .join(";");

                code.push_str(format!("{vars};").as_str());
            }
            ASTNode::ReturnExpression {
                start: _,
                expression,
            } => {
                code.push_str(
                    format!("return {};", Transpiler::transpile_expression(expression)).as_str(),
                );
            }
            ASTNode::ConstDeclaration { start: _, vars } => {
                let vars = vars
                    .iter()
                    .map(|v| format!("{}={}", v.0, Transpiler::transpile_expression(&v.1)))
                    .collect::<Vec<_>>()
                    .join(",");

                code.push_str(format!("const {vars};").as_str());
            }
            ASTNode::WhenExpression {
                start: _,
                expression,
                content,
            } => {
                let expression = Transpiler::transpile_expression(expression);
                let body = content
                    .iter()
                    .map(Transpiler::transpile_node)
                    .collect::<Vec<_>>()
                    .join("\n");

                code.push_str(format!("if ({expression}) {{\n{body}\n}}\n").as_str());
            }
            ASTNode::OrWhenExpression {
                start: _,
                expression,
                content,
            } => {
                let expression = Transpiler::transpile_expression(expression);
                let body = content
                    .iter()
                    .map(Transpiler::transpile_node)
                    .collect::<Vec<_>>()
                    .join("\n");

                code.push_str(format!("else if ({expression}) {{\n{body}\n}}\n").as_str());
            }
            ASTNode::OrExpression {
                start: _,
                content,
            } => {
                let body = content
                    .iter()
                    .map(Transpiler::transpile_node)
                    .collect::<Vec<_>>()
                    .join("\n");

                code.push_str(format!("else {{\n{body}\n}}\n").as_str());
            }
            _ => {}
        }

        code
    }

    fn transpile_expression(expression: &Expression) -> String {
        match expression {
            Expression::Value(value) => match value {
                Value::Literal(literal) => format!("\"{literal}\""),
                Value::Number(number) => number.to_string(),
            },
            Expression::Identifier(id) => id.to_string(),
            Expression::FunctionCall {
                function,
                parameters,
            } => {
                let params = parameters
                    .iter()
                    .map(|x| Transpiler::transpile_expression(x))
                    .collect::<Vec<_>>()
                    .join(",");

                format!("{function}({params})")
            }
            Expression::Binary { left, op, right } => {
                let left = Transpiler::transpile_expression(left);
                let right = Transpiler::transpile_expression(right);

                // TODO: handle different operators
                format!(
                    "({left}{}{right})",
                    match op {
                        Operator::Plus => "+",
                        Operator::Minus => "-",
                        Operator::Times => "*",
                        Operator::Divided => "/",
                        Operator::Gt => ">",
                        Operator::Lt => "<",
                        Operator::Equal => "=",
                    }
                )
            }
        }
    }
}
