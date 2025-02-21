use inquire::{Text, Select};

use anyhow::Result;

mod authenticator;
mod database;

use authenticator::User;
use database::Database;

fn prompt_username() -> Result<String> {
    // use inquire::Text;
    todo!()
}

fn prompt_password() -> Result<String> {
    // use inquire::Password;
    todo!()
}

fn prompt_code() -> Result<u32> {
    todo!()
}

fn register(db: &mut Database) -> Result<()> {
    let username: String = prompt_username()?;

    let password: String = todo!();

    let user = User::register(username, &password)?;
        
    let url = user.get_totp_url();
    qr2term::print_qr(&url).expect("QR generation error");

    let code: u32 = todo!();

    //TODO: test OTP before registering

    Ok(db.store(user)?)

}

/// Console-based authentication flow
fn authenticate(db: &mut Database) -> Result<()> {
    let username: String = todo!();

    let password: String = todo!();

    let user = db.fetch(&username).unwrap();
    // TODO: defend against side channels !

    let password_ok = user.authenticate_password(&password);

    let code: String = todo!();
    let code_ok = user.authenticate_otp(&code);

    if (password_ok && code_ok) {
        eprintln!("Welcome, {}.", user.username())
    } else {
        eprintln!("Authentication failed.")
    }
    Ok(())

}

fn main() -> Result<()> {

    let mut db: Database = Database::new();

    loop {
    let select = Select::new("What do you want to do?",
        vec!["register", "authenticate", "exit"])
        .prompt()?;

        let result = match select {
            "register" => register(&mut db),
            "authenticate" => authenticate(&mut db),
            "exit" => return Ok(()),
            _ => unreachable!(),
        };

        if let Err(e) = result {
            eprintln!("{e}");
        }

    }
}
