use nu_engine::basic_evaluation_context;
use nu_engine::whole_stream_command;
use std::error::Error;

pub fn create_default_context(interactive: bool) -> Result<EvaluationContext, Box<dyn Error>> {
    let context = basic_evaluation_context()?;
    context.add_commands(vec![whole_stream_command(Echo)]);

    Ok(context)
}
