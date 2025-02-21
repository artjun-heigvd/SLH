use futures::executor::block_on;
use read_input::prelude::*;
use regex::Regex;

mod access_control;

enum Actions {
    Exit,
    MyFiles,
    Git,
    Admin,
}

fn ask_action() -> Actions {
    println!(
        "
        Press 1 for accessing your files
        Press 2 for accessing the Git
        Press 3 for accessing the Admin console
        Press 0 to leave the software
        What is your action?",
    );
    let res: u8 = input().inside(0..=3).get();
    match res {
        0 => Actions::Exit,
        1 => Actions::MyFiles,
        2 => Actions::Git,
        3 => Actions::Admin,
        _ => panic!("Impossible values"),
    }
}

fn show_files(username: &str) {
    println!("Here are {}'s files", username);
}

fn show_git(username: &str) {
    if block_on(access_control::auth(username, GIT)) {
        println!("Here is the Git");
    } else {
        println!("You are not allowed to access the Git");
    }
}

fn show_admin(username: &str) {
    if block_on(access_control::auth(username, ADMIN)) {
        println!("Here is the admin console");
    } else {
        println!("You are not allowed to access the admin console");
    }
}

fn main() {
    println!("Welcome to our software");
    let re = Regex::new(USERNAME_REGEX).unwrap();
    let username: String = input()
        .msg("What is your username? ")
        // TODO: add input validation
        .get();
    //No authentication

    loop {
        match ask_action() {
            Actions::MyFiles => show_files(&username),
            Actions::Admin => show_admin(&username),
            Actions::Git => show_git(&username),
            Actions::Exit => {
                println!("Bye bye");
                break;
            }
        }
    }
}
