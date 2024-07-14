mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;

fn main() {
    let usr_req = get_user_response("What webserver are we building today?");

    println!("{}", usr_req);
}
