use itertools;

use common;
use config;


pub fn dump_config(config: &config::Configuration) -> String {
    let opts =
        itertools::join(config
                            .opts()
                            .iter()
                            .map(|(k, v)| {
                                     format!("{}'{}:{}'", common::take(1, common::TAB), k, v)
                                 }),
                        ",\n");
    let file = config.file();
    itertools::join(format!("(Configuration:\nOptions: '(\n{})',\nFile: '{}')", opts, file)
                        .split(common::NL),
                    format!("\n{}", common::take(1, common::TAB)).as_str())
}
