use std::rc::Weak;

pub trait Observable<O: Observer<Observes = Self::Has>> {
    type Has: PartialEq;

    fn register(&mut self, observer: Weak<O>);

    fn trigger(&mut self);

    fn set_silently(&mut self, data: Self::Has);

    fn peek(&self) -> &Self::Has;

    fn set(&mut self, data: Self::Has) {
        self.set_silently(data);
        self.trigger();
    }

    fn set_if_changed(&mut self, data: Self::Has) {
        if data != *self.peek() {
            self.set(data)
        }
    }

    fn mutate<F>(&mut self, mut f: F)
        where F: FnMut(&Self::Has) -> Self::Has
    {
        let new_value = f(self.peek());
        self.set(new_value);
    }
}

pub trait Observer {
    type Observes;

    fn update(&self, data: &Self::Observes);
}

impl<T> Observer for Box<Observer<Observes = T>> {
    type Observes = T;

    fn update(&self, data: &Self::Observes) {
        (**self).update(data);
    }
}