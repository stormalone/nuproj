use nu_engine::run_block;
use nu_engine::EvaluationContext;

pub async fn run_nu(line: String) -> String {

    let context = create_default_context(true);
    match context {
        Ok(mut ctx) => {
            // print the command to help debug unhandled errors
            println!("processing line {}", &line);
            ctx.add_commands(vec![
                whole_stream_command(random_dice::SubCommand),
                whole_stream_command(ls::Ls),
                whole_stream_command(open::Open),
                whole_stream_command(sys::Sys),
            ]);
            match parse_and_eval(&line, &mut ctx).await {
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

pub async fn parse_and_eval(line: &str, ctx: &EvaluationContext) -> Result<String, ShellError> {
    // FIXME: do we still need this?
    let line = if let Some(s) = line.strip_suffix('\n') {
        s
    } else {
        line
    };

    // TODO ensure the command whose examples we're testing is actually in the pipeline
    ctx.scope.enter_scope();
    let (classified_block, err) = nu_parser::parse(&line, 0, &ctx.scope);
    if let Some(err) = err {
        ctx.scope.exit_scope();
        return Err(err.into());
    }

    let input_stream = InputStream::empty();
    let env = ctx.get_env();
    ctx.scope.add_env(env);

    let result = run_block(&classified_block, ctx, input_stream).await;
    ctx.scope.exit_scope();

    result?.collect_string(Tag::unknown()).await.map(|x| x.item)
}
