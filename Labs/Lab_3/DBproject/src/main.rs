

pub struct UserBase {
    fname:String,
}

use bcrypt::{DEFAULT_COST, hash, verify,BcryptError};
use sqlite::Error as SqErr;

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

    pub fn pay(&self, u_from:&str, u_to:&str, amount:i64)->Result<(),UBaseErr>{
        let conn=sqlite::open(&self.fname)?;
        let mut st= conn.prepare("insert into transactions (u_from, u_to, t_date, t_amount) values(?,?,datetime(\"now\"),?);")?;
        st.bind(1,u_from)?; st.bind(2,u_to)?; st.bind(3,amount)?; st.next()?;
        Ok(()) }
}

#[cfg(test)]
mod Test;

fn main() {

}
