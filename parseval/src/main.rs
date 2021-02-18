use parseval::cli::run_nu;

pub async fn check() {
    let result = run_nu("echo 'hello'".to_string()).await;
    println!("{:#?}", result);
}

fn main() {
    println!("Hello, world!");
    futures::executor::block_on(check());
}
