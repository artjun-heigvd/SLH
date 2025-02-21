use tracing::{debug, instrument, level_filters::LevelFilter, Level};
use users::Email;

mod users;


fn init_logging() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .json()
        .init();
}

#[instrument]
fn fib(x: u32) -> u32 {
    if x < 2 {
        debug!("end of recursion");
        return x 
    }
    fib(x-1) + fib(x-2)
}

fn run_session(user: &Email) {

    let span = tracing::span!(Level::TRACE, "session", user = format!("{user}"));

    println!("Computing fib(4) for {user:?}");
    let r = fib(4);
    println!("fib(4) = {r}");

    drop(span);

}

fn main() {
    init_logging();

    let mut db = users::DB::default();

    db.register("max@example.com".into(), "1234".into())
        .expect("register max failed");
    db.register("gregoire@example.com".into(), "aaaa".into())
        .expect("register greg failed");

    db.register("InVaLiD <script>alert('xss');</script>".into(), "xxxx".into())
        .expect_err("invalid registration success");


    let max = db.login("max@example.com".into(), "1234")
        .expect("max login fail");

    run_session(&max);

    let greg = db.login("gregoire@example.com".into(), "aaaa")
        .expect("greg login fail");

    run_session(&greg);

    db.login("<script>AaAa</script>".into(), "");
    db.login("pablo@example.com".into(), "1234");
    db.login("max@example.com".into(), "WRONG");
    
}


