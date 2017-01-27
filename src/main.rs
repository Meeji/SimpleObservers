use std::rc::{Rc, Weak};



trait Observable<O: Observer<Observes = Self::Has>> {
    type Has;

    fn register_observer(&mut self, observer: Weak<O>);

    fn set(&mut self, data: Self::Has);

    fn set_without_update(&mut self, data: Self::Has);

    fn peek(&self) -> &Self::Has;

    fn mutate<F>(&mut self, mut f: F)
        where F: FnMut(&Self::Has) -> Self::Has
    {
        let new_value = f(self.peek());
        self.set(new_value);
    }
}



trait Observer {
    type Observes;

    fn update(&self, data: &Self::Observes);
}



impl<T> Observer for Box<Observer<Observes = T>> {
    type Observes = T;

    fn update(&self, data: &Self::Observes) {
        (**self).update(data);
    }
}



struct ObservableValue<T, O: Observer<Observes = T>> {
    observables: Vec<Weak<O>>,
    value: T,
}
impl<T, O: Observer<Observes = T>> ObservableValue<T, O> {
    fn new(value: T) -> ObservableValue<T, O> {
        ObservableValue {
            observables: vec![],
            value: value,
        }
    }

    fn clean(&mut self) {
        self.observables.retain(|o| o.upgrade().is_some());
    }
}
impl<T, O: Observer<Observes = T>> Observable<O> for ObservableValue<T, O> {
    type Has = T;

    fn register_observer(&mut self, observer: Weak<O>) {
        self.observables.push(observer);
    }

    fn set_without_update(&mut self, data: T) {
        self.value = data;
    }

    fn peek(&self) -> &T {
        &self.value
    }

    fn set(&mut self, data: T) {
        self.clean();

        for o in &self.observables {
            match o.upgrade() {
                Some(o) => o.update(&data),
                _ => (),
            }
        }

        self.set_without_update(data);
    }
}



struct SimpleObserver {
    name: String,
}
impl SimpleObserver {
    fn new(name: &str) -> SimpleObserver {
        SimpleObserver { name: name.to_string() }
    }
}
impl Observer for SimpleObserver {
    type Observes = usize;

    fn update(&self, data: &usize) {
        println!("{}, {:?}", self.name, *data);
    }
}



fn main() {
    {
        let mut observable = ObservableValue::new(5);

        let observer_one = Rc::new(SimpleObserver::new("obs1"));
        let observer_two = Rc::new(SimpleObserver::new("obs2"));

        observable.register_observer(Rc::downgrade(&observer_one));
        observable.register_observer(Rc::downgrade(&observer_two));
        {
            let observer_three = Rc::new(SimpleObserver::new("obs3"));
            observable.register_observer(Rc::downgrade(&observer_three));
            observable.set(6usize);
        }

        observable.set(7usize);
    }
    {
        let mut observable = ObservableValue::new(100);
        let observer_one = Rc::new(Box::new(SimpleObserver::new("obs1")) as
                                   Box<Observer<Observes = usize>>);
        let observer_two = Rc::new(Box::new(SimpleObserver::new("obs2")) as
                                   Box<Observer<Observes = usize>>);

        observable.register_observer(Rc::downgrade(&observer_one));
        observable.register_observer(Rc::downgrade(&observer_two));
        {
            let observer_three = Rc::new(Box::new(SimpleObserver::new("obs3")) as
                                         Box<Observer<Observes = usize>>);
            observable.register_observer(Rc::downgrade(&observer_three));
            observable.set(200usize);
        }

        observable.mutate(|n| n + 10);
    }
}