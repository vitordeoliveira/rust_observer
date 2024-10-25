use std::fmt::{Debug, Display};

trait Observable<T, M>
where
    T: Observer<M>,
    M: Clone + Debug + Display,
{
    fn set_message(&mut self, message: M);
    fn new() -> Self;
    fn register(&mut self, observer: T);
    fn unregister(&mut self, observer: T);
    fn notify(&mut self);
}

trait Observer<M: Clone + Debug + Display> {
    fn update(&self, message: M);
}

#[derive(PartialEq, Clone, Copy)]
struct ObserverProcess {}

struct Subject<T: Observer<M>, M: Clone + Debug + Display> {
    message: Option<M>,
    observers: Vec<T>,
}

impl<M: Clone + Debug + Display> Observer<M> for ObserverProcess {
    fn update(&self, message: M) {
        println!("message: {message}");
    }
}

impl<T, M> Observable<T, M> for Subject<T, M>
where
    T: Observer<M> + PartialEq,
    M: Clone + Debug + Display,
{
    fn new() -> Subject<T, M> {
        Subject {
            message: None,
            observers: vec![],
        }
    }

    fn set_message(&mut self, message: M) {
        self.message = Some(message);
        self.notify();
    }

    fn register(&mut self, observer: T) {
        self.observers.push(observer);
    }

    fn unregister(&mut self, observer: T) {
        let index = self.observers.iter().position(|x| *x == observer).unwrap();
        self.observers.remove(index);
    }

    fn notify(&mut self) {
        for observer in &self.observers {
            if let Some(value) = self.message.clone() {
                observer.update(value);
            };
        }
    }
}

fn main() {
    let mut subject = Subject::<ObserverProcess, String>::new();

    let a = ObserverProcess {};
    let b = ObserverProcess {};
    let c = ObserverProcess {};

    subject.register(a);
    subject.register(b);
    subject.register(c);

    subject.set_message("Hello world".to_string());

    subject.unregister(b);

    subject.set_message("Bye world".to_string());
}
