pub struct UserBase {
    fname:String,
}

use bcrypt::{DEFAULT_COST, hash, verify,BcryptError};
use sqlite::{Error as SqErr, State};
use sqlite::Value::Integer;

#[derive(Debug)]
pub enum UBaseErr{
    DbErr(SqErr),
    HashError(BcryptError)
}

impl From<SqErr> for UBaseErr{
    fn from(s:SqErr)->Self{
        UBaseErr::DbErr(s)
    }
}
impl From<BcryptError> for UBaseErr{
    fn from(b:BcryptError)->Self{
        UBaseErr::HashError(b)
    }
}

impl UserBase {

    pub fn add_user(&self, u_name:&str, p_word:&str) -> Result<(),UBaseErr>{
        let conn = sqlite::open(&self.fname)?;
        let hpass = bcrypt::hash(p_word,DEFAULT_COST)?;
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

    fn get_transactions_history(&self, u_name:&str)->Result<(),UBaseErr>{
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


}

#[cfg(test)]
mod Test;

fn main() {

    let BankBase = UserBase {
        fname: String::from("./data/users.db"),
    };

    //I added a balance table which will store the balance of users, and save_money function will
    //save money to the balance.
    BankBase.save_money("Jack",5000);
    BankBase.modified_pay("Jack","Yilong",1000);
    BankBase.get_transactions_history("Jack");
    BankBase.get_transactions_history("Yilong");
    BankBase.modified_pay("Omar","Yilong",20000);





}
