use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone,Debug)]
struct Bank{
    accounts:Arc<Mutex<Vec<i32>>>
}
impl Bank {
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for m in 0..n {
            v.push(m as i32);
        }
        Bank {
            accounts: Arc::new(Mutex::new(v)),
        }
    }
    
    fn transfer(&self, from:usize, to:usize, amount:i32)->Result<(),()> {
        let mut temp = self.accounts.lock().unwrap();
        let mut flag = 0;
        if temp.contains(&(from as i32)) && temp.contains(&(to as i32)){
            flag = 1;
        }
        if flag == 1 {
            println!("Amount of ${} transferred from account id: {} to account id: {}.", amount, from, to);
            return Ok(())
        }
        else {
            return Err(())
        }
    }
}


struct Person{
    ac_id:usize,
    buddy_id:usize,
}
impl Person{
    pub fn new(id:usize,b_id:usize)->Self{ 
        Person{
            ac_id: id,
            buddy_id: b_id
        } 
    }
}

fn main() {

    let bank = Bank::new(15);
    let mut user_vec = Vec::new();
    for i in 0..25 {
        user_vec.push(Person::new(i,i));
    }

    for i in 0..10 {
        let bank = bank.clone();
        let users = Person::new(i,i*2);
        let handle = thread::spawn(move || {
            bank.transfer(users.ac_id, users.buddy_id, 10);
        });
        handle.join().unwrap();
    }
}