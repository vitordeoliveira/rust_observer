use crate::observer::Observer;

pub trait Observable<T>
where
    T: Observer + PartialEq,
{
    fn new() -> Self;
    fn register(&mut self, observer: T);
    fn unregister(&mut self, observer: T);
    fn notify(&mut self, message: T::Message);
}
