use std::fmt::{Debug, Display};

trait Observable<T>
where
    T: Observer,
{
    fn set_message(&mut self, message: T::Message);
    fn new() -> Self;
    fn register(&mut self, observer: T);
    fn unregister(&mut self, observer: T);
    fn notify(&mut self);
}

trait Observer {
    type Message: Clone + Debug + Display;
    fn update(&self, message: Self::Message);
}

#[derive(PartialEq, Clone, Copy)]
struct ObserverProcess {}

struct Subject<T: Observer> {
    message: Option<T::Message>,
    observers: Vec<T>,
}

impl Observer for ObserverProcess {
    type Message = String;
    fn update(&self, message: Self::Message) {
        println!("message: {message}");
    }
}

impl<T> Observable<T> for Subject<T>
where
    T: Observer + PartialEq,
{
    fn new() -> Subject<T> {
        Subject {
            message: None,
            observers: vec![],
        }
    }

    fn set_message(&mut self, message: T::Message) {
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
    let mut subject = Subject::<ObserverProcess>::new();

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
