use std::rc::{Rc, Weak};



trait Observable<O: Observer<Observes = Self::Has>> {
    type Has;

    fn register_observer(&mut self, observer: Weak<O>);

    fn set(&mut self, data: Self::Has);

    fn set_without_update(&mut self, data: Self::Has);

    fn peek(&self) -> &Self::Has;
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



struct StrongObservable<O: Observer<Observes = usize>> {
    observables: Vec<Weak<O>>,
    value: usize,
}
impl<O: Observer<Observes = usize>> StrongObservable<O> {
    fn new(value: usize) -> StrongObservable<O> {
        StrongObservable {
            observables: vec![],
            value: value,
        }
    }

    fn clean(&mut self) {
        self.observables.retain(|o| o.upgrade().is_some());
    }
}
impl<O: Observer<Observes = usize>> Observable<O> for StrongObservable<O> {
    type Has = usize;

    fn register_observer(&mut self, observer: Weak<O>) {
        self.observables.push(observer);
    }

    fn set_without_update(&mut self, data: usize) {
        self.value = data;
    }

    fn peek(&self) -> &usize {
        &self.value
    }

    fn set(&mut self, data: usize) {
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



struct ObserverStruct {
    name: String,
}
impl ObserverStruct {
    fn new(name: &str) -> ObserverStruct {
        ObserverStruct { name: name.to_string() }
    }
}
impl Observer for ObserverStruct {
    type Observes = usize;

    fn update(&self, data: &usize) {
        println!("{}, {:?}", self.name, *data);
    }
}



fn main() {
    {
        let mut observable = StrongObservable::new(5);

        let observer_one = Rc::new(ObserverStruct::new("obs1"));
        let observer_two = Rc::new(ObserverStruct::new("obs2"));

        observable.register_observer(Rc::downgrade(&observer_one));
        observable.register_observer(Rc::downgrade(&observer_two));
        {
            let observer_three = Rc::new(ObserverStruct::new("obs3"));
            observable.register_observer(Rc::downgrade(&observer_three));
            observable.set(6usize);
        }

        observable.set(7usize);
    }
    {
        let mut observable = StrongObservable::new(100);
        let observer_one = Rc::new(Box::new(ObserverStruct::new("obs1")) as
                                   Box<Observer<Observes = usize>>);
        let observer_two = Rc::new(Box::new(ObserverStruct::new("obs2")) as
                                   Box<Observer<Observes = usize>>);

        observable.register_observer(Rc::downgrade(&observer_one));
        observable.register_observer(Rc::downgrade(&observer_two));
        {
            let observer_three = Rc::new(Box::new(ObserverStruct::new("obs3")) as
                                         Box<Observer<Observes = usize>>);
            observable.register_observer(Rc::downgrade(&observer_three));
            observable.set(200usize);
        }

        observable.set(300usize);
    }
}