use std::rc::Weak;
use traits::*;

pub struct ObservableValue<T, O: Observer<Observes = T>> {
    observables: Vec<Weak<O>>,
    value: T,
}

impl<T, O: Observer<Observes = T>> ObservableValue<T, O> {
    pub fn new(value: T) -> ObservableValue<T, O> {
        ObservableValue {
            observables: vec![],
            value: value,
        }
    }

    pub fn clean(&mut self) {
        self.observables.retain(|o| o.upgrade().is_some());
    }

    pub fn observables(&self) -> usize {
        self.observables.len()
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