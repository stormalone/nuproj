use nu_engine::basic_evaluation_context;
use nu_engine::whole_stream_command;
use nu_engine::EvaluationContext;
use std::error::Error;

use crate::echo::Echo;

pub fn create_default_context() -> Result<EvaluationContext, Box<dyn Error>> {
    let context = basic_evaluation_context()?;
    context.add_commands(vec![whole_stream_command(Echo)]);
    Ok(context)
}
