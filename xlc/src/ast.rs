use common;


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Assign,
    Identifier,
    Integer,
    String,
    Print,
    Separator,
    Unknown,
}


pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub location: common::SourceLocation,
}


pub enum NodeKind<'a> {
    Assignment {
        identifier: Option<String>,
        expression: Option<Node<'a>>,
    },
    Expression {
        identifier: Option<String>,
        integer: Option<i64>,
        string: Option<String>,
    },
    Print {
        expression: Option<Node<'a>>,
    },
    Program {
        statements: Nodes<'a>,
    },
    Unknown,
}


pub struct Node<'a> {
    pub kind: Box<NodeKind<'a>>,
    pub token: Option<&'a Token>,
}
impl<'a> Node<'a> {
    pub fn new(kind: Box<NodeKind<'a>>, token: Option<&'a Token>) -> Node<'a> {
        Node { kind: kind, token: token }
    }
}


pub type Tokens = common::Collection<Token>;
pub type Nodes<'a> = common::Collection<Node<'a>>;


pub trait NodeVisitor<'a> {
    type Result;

    fn begin_visit_node(&mut self, _node: &'a Node<'a>) {}
    fn visit_node(&mut self, _node: &'a Node<'a>) {}
    fn end_visit_node(&mut self, _node: &'a Node<'a>) {}
    fn should_descend_into_node(&mut self, _node: &'a Node<'a>) -> bool { true }

    fn begin_visit_assignment(&mut self, _assignment: &'a Node<'a>) {}
    fn visit_assignment(&mut self, _assignment: &'a Node<'a>) {}
    fn end_visit_assignment(&mut self, _assignment: &'a Node<'a>) {}
    fn should_descend_into_assignment(&mut self, _assignment: &'a Node<'a>) -> bool { true }

    fn begin_visit_expression(&mut self, _expression: &'a Node<'a>) {}
    fn visit_expression(&mut self, _expression: &'a Node<'a>) {}
    fn end_visit_expression(&mut self, _expression: &'a Node<'a>) {}
    fn should_descend_into_expression(&mut self, _expression: &'a Node<'a>) -> bool { true }

    fn begin_visit_print(&mut self, _print: &'a Node<'a>) {}
    fn visit_print(&mut self, _print: &'a Node<'a>) {}
    fn end_visit_print(&mut self, _print: &'a Node<'a>) {}
    fn should_descend_into_print(&mut self, _print: &'a Node<'a>) -> bool { true }

    fn begin_visit_program(&mut self, _program: &'a Node<'a>) {}
    fn visit_program(&mut self, _program: &'a Node<'a>) {}
    fn end_visit_program(&mut self, _program: &'a Node<'a>) {}
    fn should_descend_into_program(&mut self, _program: &'a Node<'a>) -> bool { true }

    fn begin_visit_unknown(&mut self, _unknown: &'a Node<'a>) {}
    fn visit_unknown(&mut self, _unknown: &'a Node<'a>) {}
    fn end_visit_unknown(&mut self, _unknown: &'a Node<'a>) {}
    fn should_descend_into_unknown(&mut self, _program: &'a Node<'a>) -> bool { true }

    fn status(self) -> common::Status<Self::Result>;
}


impl<'a> Node<'a> {

    pub fn accept<T>(&'a self, visitor: &mut NodeVisitor<'a, Result=T>) {
        match *self.kind {
            NodeKind::Assignment {..} => {
                visitor.begin_visit_node(self);
                visitor.begin_visit_assignment(self);
                visitor.visit_node(self);
                visitor.visit_assignment(self);
                self.descend_into_assignment(visitor);
                visitor.end_visit_assignment(self);
                visitor.end_visit_node(self);
            }
            NodeKind::Expression {..} => {
                visitor.begin_visit_node(self);
                visitor.begin_visit_expression(self);
                visitor.visit_node(self);
                visitor.visit_expression(self);
                self.descend_into_expression(visitor);
                visitor.end_visit_expression(self);
                visitor.end_visit_node(self);
            }
            NodeKind::Print {..} => {
                visitor.begin_visit_node(self);
                visitor.begin_visit_print(self);
                visitor.visit_node(self);
                visitor.visit_print(self);
                self.descend_into_print(visitor);
                visitor.end_visit_print(self);
                visitor.end_visit_node(self);
            }
            NodeKind::Program {..} => {
                visitor.begin_visit_node(self);
                visitor.begin_visit_program(self);
                visitor.visit_node(self);
                visitor.visit_program(self);
                self.descend_into_program(visitor);
                visitor.end_visit_program(self);
                visitor.end_visit_node(self);
            }
            NodeKind::Unknown => {
                visitor.begin_visit_node(self);
                visitor.begin_visit_unknown(self);
                visitor.visit_node(self);
                visitor.visit_unknown(self);
                self.descend_into_unknown(visitor);
                visitor.end_visit_unknown(self);
                visitor.end_visit_node(self);
            }
        }
    }

    fn descend_into_assignment<T>(&'a self, visitor: &mut NodeVisitor<'a, Result=T>) {
        let descend = visitor.should_descend_into_node(self) &&
                      visitor.should_descend_into_assignment(self);
        match *self.kind { NodeKind::Assignment { ref expression, .. } => {
            match *expression {
                Some(ref expression) if descend => expression.accept(visitor),
            _ => {}, }
        }, _ => unreachable!(), }
    }

    fn descend_into_expression<T>(&'a self, visitor: &mut NodeVisitor<'a, Result=T>) {
        let _descend = visitor.should_descend_into_node(self) &&
                       visitor.should_descend_into_expression(self);
    }

    fn descend_into_print<T>(&'a self, visitor: &mut NodeVisitor<'a, Result=T>) {
        let descend = visitor.should_descend_into_node(self) &&
                      visitor.should_descend_into_print(self);
        match *self.kind { NodeKind::Print { ref expression, .. } => {
            match *expression {
                Some(ref expression) if descend => expression.accept(visitor),
            _ => {}, }
        }, _ => unreachable!(), }
    }

    fn descend_into_program<T>(&'a self, visitor: &mut NodeVisitor<'a, Result=T>) {
        let descend = visitor.should_descend_into_node(self) &&
                      visitor.should_descend_into_program(self);
        match *self.kind { NodeKind::Program { ref statements, .. } => {
            if descend { for statement in statements.iter() {
                statement.accept(visitor);
            } }
        }, _ => unreachable!(), }
    }

    fn descend_into_unknown<T>(&'a self, visitor: &mut NodeVisitor<'a, Result=T>) {
        let _descend = visitor.should_descend_into_node(self) &&
                       visitor.should_descend_into_unknown(self);
    }

}
