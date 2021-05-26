use hamcrest2::prelude::*;

use super::*;
use hamcrest2::core::assert_that;

#[test]
pub fn test_a() {
    let player_1 = Player{
        id:001,
        first_name:String::from("Micheal"),
        last_name:String::from("Jack"),
    };
    assert_that!(player_1.last_name,type_of::<String>());
}

#[test]
pub fn test_b() {
    let player_a = Player{
        id:2020,
        first_name:String::from("WU"),
        last_name:String::from("YILONG"),
    };
    let player_b = Player{
        id:2020,
        first_name:String::from("WU"),
        last_name:String::from("YILONG"),
    };
    assert_that!(player_a.id,eq(player_b.id));
    assert_that!(player_a.first_name,eq(player_b.first_name));
    assert_that!(player_a.last_name,eq(player_b.last_name));
}