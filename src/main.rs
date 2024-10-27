use observable::Observable;
use observer::Observer;
mod observable;
mod observer;

#[derive(PartialEq, Clone, Copy)]
struct ObserverProcess {}

struct Subject<T>
where
    T: Observer,
{
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
        Subject { observers: vec![] }
    }

    fn register(&mut self, observer: T) {
        self.observers.push(observer);
    }

    fn unregister(&mut self, observer: T) {
        let index = self.observers.iter().position(|x| *x == observer).unwrap();
        self.observers.remove(index);
    }

    fn notify(&mut self, message: T::Message) {
        for observer in &self.observers {
            observer.update(message.clone());
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

    subject.notify("Hello world".to_string());

    subject.unregister(b);

    subject.notify("Bye world".to_string());
}
