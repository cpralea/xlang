extern crate argparse;
extern crate itertools;
#[macro_use]
extern crate maplit;
extern crate ref_eq;


mod common;
mod analyzer;
mod ast;
mod cdata;
mod config;
mod dumper;
mod emitter;
#[macro_use]
mod io;
mod parser;
mod tokenizer;


use std::process;


fn main() {
    let config = parse_cmd_line();
    let source = load_source(&config);
    let tokens = tokenize(&source, &config);
    let node = parse(&tokens, &source, &config);
    let cdata = analyze(&node, &source, &config);
    let code = emit_llvm(&cdata, &config);
    write_code(&code, &config);
}


fn write_code(code: &String, config: &config::Configuration) {
    if config.no_output() {
        return;
    }
    let ll = config.output();
    let result = io::Destination::to_file(&ll);
    if let Err(_) = result {
        error!("Could not create output file '{}'.", ll);
        process::exit(255);
    }
    let result = result.ok().unwrap().write(&code);
    if let Err(_) = result {
        error!("Could not write output file '{}'.", ll);
        process::exit(255);
    }
}


fn emit_llvm<'a>(block: &cdata::Block<'a>, config: &config::Configuration) -> String {
    let status = emitter::emit_llvm(block);
    let code = status.result;
    if config.verbose() {
        dump_code(&code);
    }
    assert!(status.error.is_none());
    code
}


fn analyze<'a>(node: &'a ast::Node<'a>,
               source: &io::Source,
               config: &config::Configuration)
               -> cdata::Block<'a> {
    let status = analyzer::analyze(node);
    let block = status.result;
    if config.verbose() {
        dump_cdata(&block);
    }
    if let Some(ref error) = status.error {
        print_error(source, error);
        process::exit(255);
    }
    block
}


fn parse<'a>(tokens: &'a ast::Tokens,
             source: &io::Source,
             config: &config::Configuration)
             -> ast::Node<'a> {
    let status = parser::parse(tokens);
    let node = status.result;
    if config.verbose() {
        dump_node(&node);
    }
    if let Some(ref error) = status.error {
        print_error(source, error);
        process::exit(255);
    }
    node
}


fn tokenize(source: &io::Source, config: &config::Configuration) -> ast::Tokens {
    let status = tokenizer::tokenize(source);
    let tokens = status.result;
    if config.verbose() {
        dump_tokens(&tokens);
    }
    if let Some(ref error) = status.error {
        print_error(source, error);
        process::exit(255);
    }
    tokens
}


fn load_source(config: &config::Configuration) -> io::Source {
    let file = config.file();
    let result = io::Source::from_file(file);
    if let Err(_) = result {
        error!("Could not read source file '{}'.", file);
        process::exit(255);
    }
    result.ok().unwrap()
}


fn parse_cmd_line() -> config::Configuration {
    let status = config::parse_cmd_line();
    let config = status.result;
    if config.verbose() {
        dump_config(&config);
    }
    assert!(status.error.is_none());
    config
}


fn print_error(source: &io::Source, error: &common::Error) {
    if let Some(location) = error.location {
        if let Some(line) = source.get_line(location.line) {
            println!("{}", line);
            println!("{}^", common::take(location.column - 1, " "));
        }
    }
    error!("{}", error);
}


fn dump_code(code: &String) {
    debug!("Code:");
    if !code.is_empty() {
        for line in code.split(common::NL) {
            debug!("{}{}", common::take(1, common::TAB), line);
        }
    } else {
        debug!("{}N/A", common::take(1, common::TAB));
    }
}


fn dump_cdata(block: &cdata::Block) {
    debug!("Compiler data:");

    debug!("{}Parent:", common::take(1, common::TAB));
    debug!("{}{}", common::take(2, common::TAB), dumper::dump_bare_node(block.node));

    debug!("{}Execution steps:", common::take(1, common::TAB));
    if !block.steps.is_empty() {
        for step in block.steps.iter() {
            debug!("{}{}", common::take(2, common::TAB), dumper::dump_step(step));
        }
    } else {
        debug!("{}N/A", common::take(2, common::TAB));
    }
}


fn dump_node(node: &ast::Node) {
    debug!("Parsed program:");
    for line in dumper::dump_node(node).split(common::NL) {
        debug!("{}{}", common::take(1, common::TAB), line);
    }
}


fn dump_tokens(tokens: &ast::Tokens) {
    debug!("Parsed tokens:");
    if !tokens.is_empty() {
        for token in tokens.iter() {
            debug!("{}{}", common::take(1, common::TAB), dumper::dump_token(token));
        }
    } else {
        debug!("{}N/A", common::take(1, common::TAB));
    }
}


fn dump_config(config: &config::Configuration) {
    debug!("Configuration:");
    for line in dumper::dump_config(config).split(common::NL) {
        debug!("{}{}", common::take(1, common::TAB), line);
    }
}
