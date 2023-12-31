

use super::repl_funcs::{scale_gen_desc, interval_gen};
use super::repl_funcs::{append, scale_gen, chord_gen, prepend, Context};
use repl_rs::Result as ReplResult;
use repl_rs::{crate_description, crate_name, crate_version};
use repl_rs::{initialize_repl, Repl};
use repl_rs::{Command, Parameter};


pub fn repl_shell() -> ReplResult<()> {
    let mut repl = initialize_repl!(Context::default())
        .use_completion(true)
        .add_command(
            Command::new("append", append)
                .with_parameter(Parameter::new("name").set_required(true)?)?
                .with_help("Append name to end of list"),
        )
        .add_command(
            Command::new("prepend", prepend)
                .with_parameter(Parameter::new("name").set_required(true)?)?
                .with_help("Prepend name to front of list"),
        )
        .add_command(
            Command::new("chord", chord_gen)
                .with_parameter(Parameter::new("root").set_required(true)?)?
                .with_parameter(Parameter::new("chord").set_required(true)?)?
                .with_help("Generate chord"),
        )
        .add_command(
            Command::new("scale", scale_gen)
                .with_parameter(Parameter::new("tonic").set_required(true)?)?
                .with_parameter(Parameter::new("scale").set_required(true)?)?
                .with_help("Generate scale"),
        )
        .add_command(
            Command::new("interval", interval_gen)
                .with_parameter(Parameter::new("root").set_required(true)?)?
                .with_parameter(Parameter::new("interval").set_required(true)?)?
                .with_parameter(Parameter::new("order").set_required(true)?)?
                .with_help("Generate scale"),
        )
        .add_command(
            Command::new("scale_desc", scale_gen_desc)
                .with_parameter(Parameter::new("tonic").set_required(true)?)?
                .with_parameter(Parameter::new("scale").set_required(true)?)?
                .with_help("Generate scale"),
        );
    repl.run()
}
