pub struct UserBase {
    fname:String,
}

use sqlite::{Error as SqErr, State};
use sqlite::Value::Integer;
use std::env;
use hamcrest2::len;
use std::io;
use argon2::Config;

#[derive(Debug)]
pub enum UBaseErr{
    DbErr(SqErr),
    HashError(argon2::Error)
}

impl From<SqErr> for UBaseErr{
    fn from(s:SqErr)->Self{
        UBaseErr::DbErr(s)
    }
}
impl From<argon2::Error> for UBaseErr{
    fn from(b:argon2::Error)->Self{
        UBaseErr::HashError(b)
    }
}

impl UserBase {

    pub fn add_user(&self, u_name:&str, p_word:&str) -> Result<(),UBaseErr>{
        let conn = sqlite::open(&self.fname)?;


        let salt = b"randomsalt";
        let config = Config::default();
        let hpass = argon2::hash_encoded(p_word.as_ref(), salt, &config).unwrap();

        let mut st = conn.prepare("insert into   users(u_name, p_word) values (?,?);")?;
        st.bind(1,u_name)?;
        st.bind(2,&hpass as &str)?;
        st.next()?;
        Ok(())
    }

    pub fn pay(&self, u_from:&str, u_to:&str, amount:i64)->Result<(),UBaseErr> {
        let conn=sqlite::open(&self.fname)?;
        let mut st= conn.prepare("insert into transactions (u_from, u_to, t_date, t_amount) values(?,?,datetime(\"now\"),?);")?;
        st.bind(1,u_from)?; st.bind(2,u_to)?; st.bind(3,amount)?;st.next()?;
        Ok(())
    }

    pub fn modified_pay(&self, u_from:&str, u_to:&str, amount:i64)->Result<(),UBaseErr>{

        let balance = self.get_balance(u_from);
        let balance_2 = self.get_balance(u_to);
        if balance < amount {
            println!("NO SUFFICIENT FUND")
        }
        else {
            let conn=sqlite::open(&self.fname)?;
            let mut st= conn.prepare("insert into transactions (u_from, u_to, t_date, t_amount) values(?,?,datetime(\"now\"),?);")?;
            st.bind(1,u_from)?; st.bind(2,u_to)?; st.bind(3,amount)?;st.next()?;

            let mut st = conn.prepare("update balances set balance = ? where account = ?;").unwrap();
            let result = balance - amount;
            st.bind(1,result).unwrap();
            st.bind(2,u_from).unwrap();
            st.next()?;

            let mut st = conn.prepare("update balances set balance = ? where account = ?;").unwrap();
            let result = balance_2 + amount;
            st.bind(1,result).unwrap();
            st.bind(2,u_to).unwrap();
            st.next()?;
        }
        Ok(())
    }

    pub fn get_balance(&self, account: &str) -> i64 {
        let conn = sqlite::open(&self.fname).unwrap();
        let mut st = conn
            .prepare("SELECT * FROM balances WHERE account = ?;").unwrap();
        st.bind(1,account);

        let mut balance:i64 = 0;

        while let State::Row = st.next().unwrap() {
            let account_name = st.read::<String>(0).unwrap();
            if account == account_name {
                return st.read::<String>(1).unwrap().parse::<i64>().unwrap();
            }
        }
        balance
    }

    pub fn save_money(&self, account:&str, money:i64)->Result<(),UBaseErr> {

        let balance = self.get_balance(account);
        let conn = sqlite::open(&self.fname).unwrap();

        if balance == 0 {
            let mut st = conn.prepare("insert into balances(account, balance) values (?,?);").unwrap();
            st.bind(1,account).unwrap();
            st.bind(2,money).unwrap();
            st.next()?;
        }
        else {
            let mut st = conn.prepare("update balances set balance = ? where account = ?;").unwrap();
            let result = balance + money;
            st.bind(1,result).unwrap();
            st.bind(2,account).unwrap();
            st.next()?;
        }
        Ok(())
    }

    pub fn get_transactions_history(&self, u_name:&str)->Result<(),UBaseErr>{
        let conn = sqlite::open(&self.fname).unwrap();
        let mut st = conn
            .prepare("SELECT * FROM transactions WHERE u_from = ?;").unwrap();
        st.bind(1,u_name);
        while let State::Row = st.next().unwrap() {
            let mut name = st.read::<String>(0).unwrap();

            if u_name == name {
                let u_from = name;
                let u_to = st.read::<String>(1).unwrap();
                let time = st.read::<String>(2).unwrap();
                let amount = st.read::<String>(3).unwrap();
                println!("{} sent {} from {} on {}",u_from,amount,u_to,time);
            }
        }

        let mut st_2 = conn.prepare("SELECT * FROM transactions WHERE u_to = ?;").unwrap();
        st_2.bind(1,u_name);
        while let State::Row = st_2.next().unwrap() {
            let mut name_2 = st_2.read::<String>(1).unwrap();

            if u_name == name_2 {
                let u_from = st_2.read::<String>(0).unwrap();
                let u_to = name_2;
                let time = st_2.read::<String>(2).unwrap();
                let amount = st_2.read::<String>(3).unwrap();
                println!("{} receive {} from {} on {}",u_to,amount,u_from,time);
            }
        }
        Ok(())
    }

    pub fn find_name(&self, name:&str) -> i64 {
        let conn = sqlite::open(&self.fname).unwrap();
        let mut st = conn
            .prepare("SELECT * FROM users WHERE u_name = ?;").unwrap();
        st.bind(1,name);

        let mut result:i64 = 0;
        while let State::Row = st.next().unwrap() {
            let temp_name = st.read::<String>(0).unwrap();
            if temp_name == name {
                result = 1;
            }
        }
        result
    }

    pub fn find_passord(&self, name:&str) -> String {
        let conn = sqlite::open(&self.fname).unwrap();
        let mut st = conn
            .prepare("SELECT * FROM users WHERE u_name = ?;").unwrap();
        st.bind(1,name);

        let mut password = String::new();
        while let State::Row = st.next().unwrap() {
            let temp_name = st.read::<String>(0).unwrap();
            if temp_name == name {
                password = st.read::<String>(1).unwrap();
            }
        }
        password
    }
}

#[cfg(test)]
mod Test;

fn main() {

    let BankBase = UserBase {
        fname: String::from("./data/users.db"),
    };

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Instruction does not exist");
    }
    else {
        if args[1] == String::from("new"){
            println!("Adding user {} with passord {} ...",args[2],args[3]);
            BankBase.add_user(&args[2], &args[3]);
            println!("Operation done successfully!");
        }

        else if args[1] == String::from("transfer") {
            let result = BankBase.find_name(&args[2]);
            let result_2 = BankBase.find_name(&args[3]);
            if result == 0 || result_2 == 0 {
                println!("The user doesn't exist");
            }
            else {

                println!("Please input your password:");
                let mut input = String::new();
                io::stdin().read_line(&mut input);

                let pw = input.trim();

                let temp = BankBase.find_passord(&args[2]);
                let valid = argon2::verify_encoded(&temp, (&pw).as_ref()).unwrap();
                if valid  {
                    println!("Sending money from {} to {}...",&args[2],&args[3]);
                    BankBase.modified_pay(&args[2],&args[3],args[4].parse::<i64>().unwrap());
                    println!("Operation done successfully!");
                }else {
                    println!("Wrong password")
                }
            }
        }

        else if args[1] == String::from("balance") {
            println!("Please input your password");

            let mut input = String::new();
            io::stdin().read_line(&mut input);

            let pw = input.trim();

            let temp = BankBase.find_passord(&args[2]);
            let valid = argon2::verify_encoded(&temp, (&pw).as_ref()).unwrap();

            if valid {
                println!("Balance is ${}",BankBase.get_balance(&args[2]));
                println!("Operation done successfully!");
            }
            else {
                println!("Wrong password");
            }
        }
    }
}
