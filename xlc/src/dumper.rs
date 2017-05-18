use std::fmt;

use itertools;

use common;
use ast;
use cdata;
use config;

use ast::NodeVisitor;


pub fn dump_config(config: &config::Configuration) -> String {
    let opts = itertools::join(config.opts().iter().map(|(k, v)|
        format!("{}'{}:{}'", common::take(1, common::TAB), k, v)), ",\n");
    let file = config.file();
    itertools::join(
        format!("(Configuration:\nOptions: '(\n{})',\nFile: '{}')", opts, file)
    .split(common::NL), format!("\n{}", common::take(1, common::TAB)).as_str())
}


pub fn dump_token(token: &ast::Token) -> String {
    format!("(Token: Value: '{}', {:?}@{})", token.value, token.kind, token.location)
}


pub fn dump_node<'a>(node: &'a ast::Node<'a>) -> String {
    let mut dumper = NodeDumper::new();
    node.accept(&mut dumper);
    dumper.status().result.clone()
}


pub fn dump_bare_node<'a>(node: &'a ast::Node<'a>) -> String {
    let header = dump_generic_node_header(node);
    let footer = dump_generic_node_footer(node);
    format!("{}{}", header, footer)
}


pub fn dump_step<'a>(step: &'a cdata::Step<'a>) -> String {
    format!("{}: {:?}", dump_bare_node(step.node), step.kind)
}


struct NodeDumper {
    level: usize,
    status: common::Status<String>,
}


impl NodeDumper {

    fn new() -> NodeDumper {
        NodeDumper {
            level: 0,
            status: common::Status { result: String::new(), error: None } }
    }

    fn push(buffer: &mut String, string: &str) {
        buffer.push_str(string);
    }

    fn ipush(buffer: &mut String, string: &str, level: usize) {
        Self::push(buffer, common::take(level, common::TAB).as_str());
        Self::push(buffer, string);
    }

    fn pushln(buffer: &mut String, string: &str) {
        Self::push(buffer, string);
        Self::push(buffer, common::NL);
    }

    fn ipushln(buffer: &mut String, string: &str, level: usize) {
        Self::ipush(buffer, string, level);
        Self::push(buffer, common::NL);
    }

}

impl<'a> ast::NodeVisitor<'a> for NodeDumper {
    type Result = String;

    fn should_descend_into_node(&mut self, _node: &'a ast::Node<'a>) -> bool { false }

    fn begin_visit_node(&mut self, node: &'a ast::Node<'a>) {
        Self::ipushln(&mut self.status.result, dump_generic_node_header(node).as_str(), self.level);
        self.level += 1;
    }

    fn end_visit_node(&mut self, node: &'a ast::Node<'a>) {
        self.level -= 1;
        Self::push(&mut self.status.result, dump_generic_node_footer(node).as_str());
    }

    fn visit_assignment(&mut self, assignment: &'a ast::Node<'a>) {
        match *assignment.kind {
            ast::NodeKind::Assignment { ref identifier, ref expression } => {
                self.println_data(identifier, "Identifier");
                self.print_child(expression, "Expression");
            },
            _ => unreachable!(),
        }
    }

    fn visit_expression(&mut self, expression: &'a ast::Node<'a>) {
        match *expression.kind {
            ast::NodeKind::Expression { ref identifier, ref integer, ref string } => {
                self.println_data(identifier, "Identifier");
                self.println_data(integer, "Integer");
                self.print_data(string, "String");
            },
            _ => unreachable!(),
        }
    }

    fn visit_print(&mut self, print: &'a ast::Node<'a>) {
        match *print.kind {
            ast::NodeKind::Print { ref expression } =>
                self.print_child(expression, "Expression"),
            _ => unreachable!(),
        }
    }

    fn visit_program(&mut self, program: &'a ast::Node<'a>) {
        match *program.kind {
            ast::NodeKind::Program { ref statements } =>
                self.print_children(statements, "Statements"),
            _ => unreachable!(),
        }
    }

    fn status(self) -> common::Status<Self::Result> {
        self.status
    }

}


fn dump_generic_node_header<'a>(node: &'a ast::Node<'a>) -> String {
    let kind = match *node.kind {
        ast::NodeKind::Assignment {..}  => "Assignment",
        ast::NodeKind::Expression {..}  => "Expression",
        ast::NodeKind::Print {..}       => "Print",
        ast::NodeKind::Program {..}     => "Program",
        ast::NodeKind::Unknown          => "Unknown",
    };
    let token = match node.token {
        Some(token) => dump_token(token),
        None        => String::from("N/A"),
    };
    format!("(Node: {}, @{}", kind, token.as_str())
}


fn dump_generic_node_footer<'a>(_node: &'a ast::Node<'a>) -> String {
    String::from(")")
}


impl NodeDumper {

    fn print_data<T: fmt::Display>(&mut self, data: &Option<T>, label: &str) {
        let data = match *data {
            Some(ref data) => format!("{}: {}", label, data),
            None           => format!("{}: N/A", label),
        };
        Self::ipush(&mut self.status.result, data.as_str(), self.level)
    }

    fn println_data<T: fmt::Display>(&mut self, data: &Option<T>, label: &str) {
        self.print_data(data, label);
        Self::pushln(&mut self.status.result, "");
    }

    fn print_child<'a>(&mut self, child: &Option<ast::Node<'a>>, label: &str) {
        Self::ipushln(&mut self.status.result, format!("{}:", label).as_str(), self.level);
        match *child {
            Some(ref child) => {
                self.level += 1;
                child.accept(self);
                self.level -= 1;
            },
            None => Self::ipush(&mut self.status.result, "N/A", self.level + 1),
        }
    }

    fn print_children<'a>(&mut self, children: &ast::Nodes<'a>, label: &str) {
        if !children.is_empty() {
            Self::ipush(&mut self.status.result, format!("{}:", label).as_str(), self.level);
            for child in children.iter() {
                Self::pushln(&mut self.status.result, "");
                self.level += 1;
                child.accept(self);
                self.level -= 1;
            }
        } else {
            Self::ipushln(&mut self.status.result, format!("{}:", label).as_str(), self.level);
            Self::ipush(&mut self.status.result, "N/A", self.level + 1);
        }
    }

}