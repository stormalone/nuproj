use serde::Serialize;

use nu_source::Tag;
use nu_stream::InputStream;

use nu_engine::run_block;
use nu_engine::whole_stream_command;
use nu_engine::EvaluationContext;
use nu_errors::ShellError;

use nu_parser::ParserScope;
use nu_protocol::hir::ExternalRedirection;

use crate::default_context::create_default_context;
use crate::echo;

#[derive(Serialize)]
enum OkError {
    Ok(String),
    Error(ShellError),
    InternalError(String),
}

pub async fn run_nu(line: String) -> String {
    let context = create_default_context();
    match context {
        Ok(ctx) => {
            // print the command to help debug unhandled errors
            println!("processing line {}", &line);
            ctx.add_commands(vec![whole_stream_command(echo::Echo)]);
            match parse_and_eval(&line, &ctx) {
                Ok(val) => match serde_json::to_string(&OkError::Ok(val)) {
                    Ok(output) => output,
                    Err(e) => format!("Error converting to json: {:?}", e),
                },
                Err(e) => match serde_json::to_string(&OkError::Error(e)) {
                    Ok(output) => output,
                    Err(e) => format!("Error converting to json: {:?}", e),
                },
            }
        }
        Err(e) => match serde_json::to_string(&OkError::InternalError(format!("{}", e))) {
            Ok(output) => output,
            Err(e) => format!("Error converting to json: {:?}", e),
        },
    }
}

pub fn parse_and_eval(line: &str, ctx: &EvaluationContext) -> Result<String, ShellError> {
    // FIXME: do we still need this?
    let line = if let Some(s) = line.strip_suffix('\n') {
        s
    } else {
        line
    };

    // TODO ensure the command whose examples we're testing is actually in the pipeline
    ctx.scope.enter_scope();
    let (classified_block, err) = nu_parser::parse(line, 0, &ctx.scope);
    if let Some(err) = err {
        ctx.scope.exit_scope();
        return Err(err.into());
    }

    let input_stream = InputStream::empty();

    let result = run_block(
        &classified_block,
        ctx,
        input_stream,
        ExternalRedirection::Stdout,
    );
    ctx.scope.exit_scope();

    result?.collect_string(Tag::unknown()).map(|x| x.item)
}
