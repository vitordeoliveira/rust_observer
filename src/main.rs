trait Observable<T: Observer> {
    fn register(&mut self, observer: T);
    fn unregister(&mut self, observer: T);
    fn notify(&mut self);
}

trait Observer {
    fn update(&self);
}

struct ObserverProcess {}

struct Subject<T: Observer> {
    observers: Vec<T>,
}

impl Observer for ObserverProcess {
    fn update(&self) {
        println!("update on me")
    }
}

impl<T: Observer> Observable<T> for Subject<T> {
    fn register(&mut self, observer: T) {
        todo!()
    }

    fn unregister(&mut self, observer: T) {
        todo!()
    }

    fn notify(&mut self) {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
