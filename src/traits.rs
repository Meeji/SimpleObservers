use std::rc::Weak;

pub trait Observable<O: Observer<Observes = Self::Has>> {
    type Has;

    fn register(&mut self, observer: Weak<O>);

    fn update(&mut self);

    fn set_without_update(&mut self, data: Self::Has);

    fn peek(&self) -> &Self::Has;

    fn set(&mut self, data: Self::Has) {
        self.set_without_update(data);
        self.update();
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