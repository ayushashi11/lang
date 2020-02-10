#[derive(Clone)]
struct Node<T>{
    dat:T,
    next:Link<T>
}
type Link<T> = Option<Box<Node<T>>>;
#[derive(Clone)]
pub struct LList<T>{
    head:Link<T>
}
impl<T> LList<T>{
    pub fn new() -> Self{
        LList{head:None}
    }
    pub fn push(&mut self, elem:T){
        let new_node = Box::new(Node{
            dat:elem,
            next:None
        });
        let mut curr=self.head;
        loop{
            match curr{
                Some(node)=>curr=&**node.next,
                
            }
        }
    }
    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map(|node| {
            self.head = node.next;
            node.dat
        })
    }
    pub fn iter<'a>(&self) -> Iter<'_,T>{
        Iter{next: self.head.as_ref().map(|node| &**node)}
    }
}
#[derive(Clone)]
pub struct Iter<'a,T>{
    next: Option<&'a Node<T>>
}
impl<T> Drop for LList<T>{
    fn drop(&mut self){
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link{
            cur_link = boxed_node.next.take();
        }
    }
}
impl<'a,T> Iterator for Iter<'a,T>{
    type Item=&'a T;
    fn next(&mut self) -> Option<Self::Item>{
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.dat
        })
    }
}
#[cfg(tests)]
mod tests{
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    fn test_pop(){
        let mut ll:this::LList<f32>= this::LList::new();
        ll.push(3.0);
        assert_eq!(ll.pop(),Some(3.0));
    }
    fn test_iter(){
        let mut ll:this::LList<f32>= this::LList::new();
        ll.push(3.0);ll.push(2.0);ll.push(1.0);
        let mut iter=ll.iter();
        assert_eq!(iter.next(), Some(&3.0));
        assert_eq!(iter.next(), Some(&2.0));
        assert_eq!(iter.next(), Some(&1.0));
    }
}
