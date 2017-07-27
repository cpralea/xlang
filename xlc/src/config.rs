use std::collections;

use argparse;

use common;


const OPT_VERBOSE: &str = "verbose";
const OPT_NO_OUTPUT: &str = "no-output";
const OPT_OUTPUT: &str = "output";


pub struct Configuration {
    opts: collections::BTreeMap<String, String>,
    file: Option<String>,
}


impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            opts: collections::BTreeMap::new(),
            file: None,
        }
    }
    pub fn verbose(&self) -> bool {
        match self.opts.get(OPT_VERBOSE) {
            None => false,
            Some(value) => value == "true",
        }
    }
    pub fn no_output(&self) -> bool {
        match self.opts.get(OPT_NO_OUTPUT) {
            None => false,
            Some(value) => value == "true",
        }
    }
    pub fn output(&self) -> &String {
        match self.opts.get(OPT_OUTPUT) {
            None => unreachable!(),
            Some(value) => value,
        }
    }
    pub fn file(&self) -> &String {
        match self.file {
            None => unreachable!(),
            Some(ref value) => value,
        }
    }
    pub fn opts(&self) -> &collections::BTreeMap<String, String> {
        &self.opts
    }
}


pub fn parse_cmd_line() -> common::Status<Configuration> {
    let (mut verbose, mut no_output, mut output, mut file) =
        (false, false, None::<String>, String::new());
    {
        let mut parser = argparse::ArgumentParser::new();
        parser.set_description("X language compiler. Converts X source code into LLVM IR.");
        parser.refer(&mut verbose).add_option(
            &["-v", "--verbose"],
            argparse::StoreTrue,
            "verbose compiler output",
        );
        parser.refer(&mut no_output).add_option(
            &["--no-output"],
            argparse::StoreTrue,
            "turn off compiler output so that no LL_FILE file will be created",
        );
        parser
            .refer(&mut output)
            .add_option(
                &["-o", "--output"],
                argparse::StoreOption,
                "LLVM IR output file name; defaults to FILE.ll",
            )
            .metavar("LL_FILE");
        parser
            .refer(&mut file)
            .add_argument("XL_FILE", argparse::Store, "X source file name")
            .required();
        parser.parse_args_or_exit();
    }

    let mut config = Configuration::new();
    config.opts.insert(String::from(OPT_VERBOSE), format!("{}", verbose));
    config.opts.insert(String::from(OPT_NO_OUTPUT), format!("{}", no_output));
    config.opts.insert(
        String::from(OPT_OUTPUT),
        match output {
            Some(output) => output,
            None => {
                format!(
                    "{}.ll",
                    match file.ends_with(".xl") {
                        true => file.rsplitn(2, ".xl").nth(1).unwrap(),
                        false => file.as_str(),
                    }
                )
            }
        },
    );
    config.file = Some(file);

    common::Status {
        result: config,
        error: None,
    }
}
