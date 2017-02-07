Trying to add an output type:
```
pub trait Monad<O> where
    O: Monad<>
{
    type Item;
    type BindItem;
    fn ret(Self::Item) -> Self;
    fn bind<F,B>(self, f:F) -> O where
        F: FnOnce(Self::Item) -> 
    ;
}

impl<A,B> Monad<Option<B>,B> for Option<A> 
{
    type Item = A;
    fn ret(a: Self::Item) -> Self { Some(a) }
    fn bind<F,B>(self,f:F) -> Option<B> where
        F: FnOnce(Self::Item) -> Option<B>,
    {
        self.and_then(f);
    }
    
}

fn main() {
    let a = Some(2usize);
    assert_eq!(Some(5), a.bind(|t|Option::ret(t+3)));
}
```