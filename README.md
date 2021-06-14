
## parseval

This is the most simple infrastructure needed with one
nushell command called **echo**

This is a bare bones code set to get nushell up and running.

This code does not contain any core nushell crates..

The core nushell crates are located
[here in nume.](https://github.com/stormasm/nume)

There are 2 branches of this code, one branch points to the
nushell code and the other branch points to the nume crates
which are mostly a copy of the nushell code with some extra
debugging information.

```
let (classified_block, err) = nu_parser::parse(&line, 0, &ctx.scope);
let result = run_block(&classified_block, ctx, input_stream).await;
```

##### branches

* x0.32.1
