use observable::Observable;
use observer::Observer;
mod observable;
mod observer;

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
    T: Observer,
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
    let mut subject: Subject<ObserverProcess> = Subject::new();

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
