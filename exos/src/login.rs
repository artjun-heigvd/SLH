use read_input::prelude::*;

fn query_db(username: &str, password: &str) -> bool {
    //Checks whether the username/password pair is valid in the DB
    //Code non fourni mais sans vuln
}



fn login(logged: &mut bool) -> String {
    *logged = false;
    let mut ok = false;
    let mut username = String::new();
    while !ok {
        username = input::<String>().msg("Username: ").get();
        let password = input::<String>().msg("Password: ").get();
        if query_db(&username, &password) {
            *logged = true;
        } else {
            println!("Invalid username/password");
        }
        println!(
            "Your current username is {}. Is this ok? (YES/NO)",
            username
        );
        if input::<String>().get() == "YES" {
            ok = true;
        }
    }
    username
}

fn main() {
    let mut logged = false;
    let username = login(&mut logged);
    if logged && username == "admin" {
        println!("You enter the admin zone");
    }
    else {
        println!("Welcome");
    }
}
