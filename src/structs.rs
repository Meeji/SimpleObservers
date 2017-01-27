use std::rc::Weak;
use traits::*;

pub struct ObservableValue<T: PartialEq, O: Observer<Observes = T>> {
    observables: Vec<Weak<O>>,
    value: T,
}

impl<T: PartialEq, O: Observer<Observes = T>> ObservableValue<T, O> {
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

impl<T: PartialEq, O: Observer<Observes = T>> Observable<O> for ObservableValue<T, O> {
    type Has = T;

    fn register(&mut self, observer: Weak<O>) {
        self.observables.push(observer);
    }

    fn set_silently(&mut self, data: T) {
        self.value = data;
    }

    fn peek(&self) -> &T {
        &self.value
    }

    fn trigger(&mut self) {
        self.clean();

        for o in &self.observables {
            if let Some(o) = o.upgrade() {
                o.update(&self.value);
            }
        }
    }
}