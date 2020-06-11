#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate game_back_end;

// use game_back_end::qa::static_rocket_route_info_for_question;
use game_back_end::datetime::static_rocket_route_info_for_test_time;
use game_back_end::user::static_rocket_route_info_for_save_player_data;
use game_back_end::datetime::static_rocket_route_info_for_return_players;
use game_back_end::datetime::static_rocket_route_info_for_return_top_scorer;
use game_back_end::datetime::static_rocket_route_info_for_return_winner;

use game_back_end::datetime::{Duration, query_top_scorer, find_winner};
use game_back_end::models::{PlayerQue};

extern crate rocket_cors;

extern crate userInfo;
// use userInfo::get_user_by_name_password;
use game_back_end::qa::save_question_to_db;
use game_back_end::qa::random_question;
use game_back_end::qa::random_answer;
use game_back_end::qa::static_rocket_route_info_for_question_for_front_end;

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {

    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    rocket::ignite()
        // .mount("/", routes![question])
        // .mount("/", routes![test_time])
        // .mount("/", routes![save_player_data])
        // .mount("/", routes![return_players])
        // .mount("/", routes![return_top_scorer])
        // .mount("/", routes![return_winner])   
        .mount("/", routes![question_for_front_end])
        .attach(cors)
        .launch();


    // let name = String::from("shing");
    // let password = String::from("123");

    // let result = get_user_by_name_password(name, password).unwrap();

    // println!("{:#?}", result);        

    // save_question_to_db();

    // let value = random_question();

    // let mut rng = thread_rng();
    // let mut array = [1, 2, 3, 4, 5, 6, 7];
    // println!("before shuffle: {:#?}", array);
    // array.shuffle(&mut rng);
    // println!("after shuffle: {:#?}", array);

    // println!("{:#?}", value);

    // let value1 = random_answer(value.unwrap());

    // println!("{:#?}", value1);
}
