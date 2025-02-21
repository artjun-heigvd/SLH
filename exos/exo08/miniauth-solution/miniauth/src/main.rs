use inquire::{Password, Select, Text};

use anyhow::{anyhow, Result};

mod authenticator;
mod database;

use authenticator::User;
use database::Database;

fn prompt_username() -> Result<String> {
    Ok(Text::new("Enter username:")
        .prompt()?)
}

fn prompt_password(repeat: bool) -> Result<String> {
    let pw = Password::new("Enter password:");
    let pw = if repeat { pw } else { pw.without_confirmation() };
    Ok(pw.prompt()?)
}

fn prompt_code() -> Result<String> {
    Ok(Text::new("enter OTP code")
       .prompt()?)
}

fn register(db: &mut Database) -> Result<()> {
    let username: String = prompt_username()?;

    let password: String = prompt_password(true)?;

    let user = User::register(username, &password)?;
        
    let url = user.get_totp_url();
    eprintln!("{url}");
    qr2term::print_qr(&url).expect("QR generation error");

    let code: String = prompt_code()?;
    if !user.authenticate_otp(&code) {
        return Err(anyhow!("Invalid OTP code"))
    }

    Ok(db.store(user)?)

}

/// Console-based authentication flow
fn authenticate(db: &mut Database) -> Result<()> {
    let username: String = prompt_username()?;

    let password: String = prompt_password(false)?;
    let fake_user = User::default();

    let (valid_user, user) = db.fetch(&username).map(|u| (true, u)).unwrap_or((false, &fake_user));
    // TODO: defend against side channels !

    let password_ok = user.authenticate_password(&password);

    let code: String = prompt_code()?;
    let code_ok = user.authenticate_otp(&code);

    if valid_user && password_ok && code_ok {
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
