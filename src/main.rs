trait Observable<T: Observer> {
    fn set_message(&mut self, message: String);
    fn new() -> Self;
    fn register(&mut self, observer: T);
    fn unregister(&mut self, observer: T);
    fn notify(&mut self);
}

trait Observer {
    fn update(&self, message: String);
}

#[derive(PartialEq, Clone, Copy)]
struct ObserverProcess {}

struct Subject<T: Observer> {
    message: String,
    observers: Vec<T>,
}

impl Observer for ObserverProcess {
    fn update(&self, message: String) {
        println!("message: {message}");
    }
}

impl<T: Observer + PartialEq> Observable<T> for Subject<T> {
    fn new() -> Subject<T> {
        Subject {
            message: "".to_string(),
            observers: vec![],
        }
    }

    fn set_message(&mut self, message: String) {
        self.message = message;
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
            observer.update(self.message.clone());
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
