use tracing::{debug, instrument, Level};
use users::Email;

mod users;


fn init_logging() {
    // TODO
}

// TODO Instrument this
fn fib(x: u32) -> u32 {
    if x < 2 {
        // TODO log the base case 
        return x 
    }
    fib(x-1) + fib(x-2)
}

fn run_session(user: &Email) {

    // TODO Enter a user session span

    // TODO replace println with tracing
    println!("Computing fib(4) for {user:?}");
    let r = fib(4);
    println!("fib(4) = {r}");

    // TODO finish the span

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


