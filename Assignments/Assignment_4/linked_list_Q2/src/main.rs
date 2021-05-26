#[derive(Debug, PartialEq)]

pub enum LinkedList<T>{
    Tail,
    Head(List<T>),
}
use self::LinkedList::*;
use std::io::empty;
use im::List;
use im::list::cons;

impl<T> LinkedList<T>{
    pub fn empty() -> Self {
        LinkedList::Tail
    }
    pub fn new(t:T) -> Self {
        LinkedList::Head(cons(t,List::new()))
    }
    pub fn push(self, t:T) -> Self {
        match self{
            LinkedList::Tail => {
                LinkedList::new(t)
            }
            LinkedList::Head(l) => {
                LinkedList::Head(cons(t,l))
            }
        }
    }
    pub fn push_back(&mut self, t:T)  {
        match self{
            LinkedList::Tail => {
                *self = LinkedList::new(t);
            }
            LinkedList::Head(ref mut temp) => {
                *temp = temp.push_back(t);
            }
        }
    }


    //add your code here:
}

#[cfg(test)]
mod tests{
    use super::*;
    use im::list::ListNode::Cons;

    #[test]
    fn it_works(){
        let mut l = LinkedList::new(3);
        l= l.push(4);


        println!("{:#?}",l);
        assert_eq!(l,Head(cons(4, cons(3, List::new()))));


        l.push_back(2);
        assert_eq!(l,Head(cons(4,(cons(3,cons(2,List::new( )))))));
        //println!("{:#?}",l);

    }
}
fn main(){

}