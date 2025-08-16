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
                content,
            } => {
                let body = Transpiler::transpile_block(content);

                code.push_str(format!("function {name}(param) {{\n{body}\n}}\n").as_str());
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
                parameter,
            } => {
                let function = Transpiler::transpile_expression(function);
                let mut parameter = Transpiler::transpile_expression(parameter);

                if parameter == "{}" {
                    parameter = String::new();
                }

                format!("{function}({parameter})")
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
                        Operator::Equal => "==",
                    }
                )
            }
            Expression::ArrayExpression(items) => {
                let items = items
                    .iter()
                    .map(|x| Transpiler::transpile_expression(x))
                    .collect::<Vec<_>>()
                    .join(",");

                format!("[{items}]")
            }
            Expression::ObjectExpression(hashmap) => {
                let obj = hashmap
                    .iter()
                    .map(|x| format!("{}: {}", x.0, Transpiler::transpile_expression(x.1)))
                    .collect::<Vec<_>>()
                    .join(",");

                format!("{{{obj}}}")
            }
            Expression::PropertyAccess { object, property } => {
                let property = Transpiler::transpile_expression(property);
                format!("{object}.{property}")
            }
            Expression::ArrayIndex { array, index } => {
                let array = Transpiler::transpile_expression(array);
                let index = Transpiler::transpile_expression(index);

                format!("{array}[{index}]")
            }
        }
    }

    fn transpile_block(nodes: &[ASTNode]) -> String {
        let mut code = String::new();
        let mut i = 0;

        while i < nodes.len() {
            match &nodes[i] {
                ASTNode::WhenExpression {
                    expression,
                    content,
                    ..
                } => {
                    code.push_str(&format!(
                        "if ({}) {{\n",
                        Transpiler::transpile_expression(expression)
                    ));
                    code.push_str(&Transpiler::transpile_block(content));
                    code.push_str("}\n");

                    i += 1;
                    while i < nodes.len() {
                        match &nodes[i] {
                            ASTNode::OrWhenExpression {
                                expression,
                                content,
                                ..
                            } => {
                                code.push_str(&format!(
                                    "else if ({}) {{\n",
                                    Transpiler::transpile_expression(expression)
                                ));
                                code.push_str(&Transpiler::transpile_block(content));
                                code.push_str("}\n");
                                i += 1;
                            }
                            ASTNode::OrExpression { content, .. } => {
                                code.push_str("else {\n");
                                code.push_str(&Transpiler::transpile_block(content));
                                code.push_str("}\n");
                                i += 1;
                                break;
                            }
                            _ => break,
                        }
                    }
                }
                other => {
                    code.push_str(&Transpiler::transpile_node(other));
                    code.push('\n');
                    i += 1;
                }
            }
        }

        code
    }
}
