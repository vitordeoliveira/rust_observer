use std::fmt::{Debug, Display};

pub trait Observer: PartialEq {
    type Message: Clone + Debug + Display;
    fn update(&self, message: Self::Message);
}
