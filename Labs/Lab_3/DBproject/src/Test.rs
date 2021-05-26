#[cfg(test)] #[macro_use]
use hamcrest2::prelude::*;
use hamcrest2::core::assert_that;

mod tests {
    use super::*;

    use sqlite::State;
    use bcrypt::verify;
    use crate::UserBase;

    #[test]
    pub fn test_functionality() {
        let BankBase = UserBase {
            fname: String::from("./data/users.db"),
        };
        let u_name = "Omar";
        let p_word = "666666";

        // test if the functions' arguments are of correct types
        assert_that!(u_name, is(type_of::<&str>()));
        assert_that!(p_word, is(type_of::<&str>()));

        BankBase.add_user(u_name, p_word);
        let connection = sqlite::open(&BankBase.fname).unwrap();
        let mut statement = connection
            .prepare("SELECT * FROM users WHERE u_name = ?;")
            .unwrap();

        statement.bind(1, "Omar").unwrap();

        while let State::Row = statement.next().unwrap() {
            assert_that!(String::from(u_name), is(equal_to(statement.read::<String>(0).unwrap())));
            let valid = verify(p_word, &statement.read::<String>(1).unwrap());
            assert_that!(valid.unwrap(), is(true));
        }

        BankBase.add_user("Yilong","666666");

        let u_from = "Yilong";
        let u_to = "Omar";
        let amount :i64 = 10000;

        // test if the functions' arguments are of correct types
        assert_that!(u_from, is(type_of::<&str>()));
        assert_that!(u_to, is(type_of::<&str>()));
        assert_that!(amount, is(type_of::<i64>()));

        BankBase.pay(u_from, u_to, amount).unwrap();
        let mut statement = connection
            .prepare("SELECT * FROM transactions WHERE u_from = ? and u_to = ?;")
            .unwrap();
        statement.bind(1, u_from).unwrap();
        statement.bind(2, u_to).unwrap();

        while let State::Row = statement.next().unwrap() {
            assert_that!(
                String::from(u_from),
                is(equal_to(statement.read::<String>(0).unwrap()))
            );
            assert_that!(String::from(u_to), is(equal_to(statement.read::<String>(1).unwrap())));
            assert_that!(amount.to_string(), is(equal_to(statement.read::<String>(3).unwrap())));
        }
    }
}