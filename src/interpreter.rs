pub mod interpreter {
    use std::collections::HashMap;

    use crate::ast::Node;    
    use crate::operators::{
        BinOpcode,
        RelOpcode,
        LogOpcode
    };

    #[derive(Debug, PartialEq)]
    enum Value {
        Number(i32),
        Bool(bool),
        Var(String),
        None,
    }

    pub fn interp(mut ast: Vec<Box<Node>>) {
        let mut vars = HashMap::new();
        for node in ast.drain(..) {
            // println!("node = {:#?}", node);
            visit(node, &mut vars);
        }
        println!("vars = {:#?}", vars);
    }

    fn visit(node: Box<Node>, map: &mut HashMap<String, Value>) -> Value {
        match *node {
            Node::Number(num) => Value::Number(num),
            Node::Bool(b) => Value::Bool(b),
            Node::BinOp(left, op, right) => eval_bin_op(visit(left, map), op, visit(right, map)),
            Node::RelOp(left, op, right) => eval_rel_op(visit(left, map), op, visit(right, map)),
            Node::LogOp(left, op, right) => eval_log_op(visit(left, map), op, visit(right, map)),
            Node::Let(var, expr) => assign_var(var, visit(expr, map), map),
            Node::Print(text) => {
                println!("{:#?}", visit(text, map));
                Value::None
            }
            _ => panic!("Node not supported: {:?}", *node)
        }
    }

    fn assign_var(var: Box<Node>, expr: Value, map: &mut HashMap<String, Value>) -> Value {
        let name = match *var {
            Node::VarBinding(var, var_type) => {
                match *var {
                    Node::Var(name) => name,
                    _ => panic!("assign_var: Var node does not contain name")
                }
            },
            _ => panic!("assign_var VAR no name")
        };

        map.insert(name, expr);

        Value::Number(5)
    }

    // TODO: add floats to grammar and handle them here
    fn eval_bin_op(left: Value, op: BinOpcode, right: Value) -> Value {
        let l = match left {
            Value::Number(num) => num,
            _ => panic!("eval_bin_op LEFT no number")
        };

        let r = match right {
            Value::Number(num) => num,
            _ => panic!("eval_bin_op RIGHT no number")
        };

        match op {
            BinOpcode::Add => Value::Number(l + r),
            BinOpcode::Sub => Value::Number(l - r),
            BinOpcode::Div => Value::Number(l / r),
            BinOpcode::Mul => Value::Number(l * r)
        }
    }

    fn eval_rel_op(left: Value, op: RelOpcode, right: Value) -> Value {
        match (left, right) {
            (Value::Number(l_num), Value::Number(r_num)) => eval_num_rel_op(l_num, op, r_num),
            (Value::Bool(l_bool), Value::Bool(r_bool)) => eval_bool_rel_op(l_bool, op, r_bool),
            _ => panic!("eval_rel_op left and right not same type")
        }
    }

    fn eval_num_rel_op(left: i32, op: RelOpcode, right: i32) -> Value {
        match op {
            RelOpcode::EQ => Value::Bool(left == right),
            RelOpcode::NEQ => Value::Bool(left != right),
            RelOpcode::GT => Value::Bool(left > right),
            RelOpcode::LT => Value::Bool(left < right),
            RelOpcode::GEQ => Value::Bool(left >= right),
            RelOpcode::LEQ => Value::Bool(left <= right),
            _ => panic!("eval_num_rel_op OPERATION not valid for numbers")
        }
    }

    fn eval_bool_rel_op(left: bool, op: RelOpcode, right: bool) -> Value {
        match op {
            RelOpcode::EQ => Value::Bool(left == right),
            RelOpcode::NEQ => Value::Bool(left != right),
            _ => panic!("eval_bool_rel_op OPERATION not valid for booleans")
        }
    }

    fn eval_log_op(left: Value, op: LogOpcode, right: Value) -> Value {
        let (l, r) = match (left, right) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => (l_bool, r_bool),
            _ => panic!("eval_log_op LEFT and RIGHT not both booleans")
        };

        match op {
            LogOpcode::AND => Value::Bool(l && r),
            LogOpcode::OR => Value::Bool(l || r)
        }
    }

}