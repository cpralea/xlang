use cdata;

use super::node;


pub fn dump_step<'a>(step: &'a cdata::Step<'a>) -> String {
    format!("{}: {:?}", node::dump_bare_node(step.node), step.kind)
}
