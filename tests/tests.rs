#[cfg(test)]
mod tests {
    extern crate simple_observables;
    use self::simple_observables::traits::{Observer, Observable};
    use self::simple_observables::structs::ObservableValue;
    use std::rc::Rc;
    use std::cell::RefCell;

    struct SimpleObserver {
        updates: RefCell<Vec<usize>>,
    }

    impl SimpleObserver {
        fn new() -> SimpleObserver {
            SimpleObserver { updates: RefCell::new(Vec::new()) }
        }
    }

    impl Observer for SimpleObserver {
        type Observes = usize;

        fn update(&self, data: &usize) {
            self.updates.borrow_mut().push(*data);
        }
    }

    #[test]
    fn updates_subscribers() {
        let mut observable = ObservableValue::new(5usize);
        let observer = Rc::new(SimpleObserver::new());
        observable.register_observer(Rc::downgrade(&observer));

        {
            let observer_2 = Rc::new(SimpleObserver::new());
            observable.register_observer(Rc::downgrade(&observer_2));
            observable.clean();

            // Both registered
            assert_eq!(observable.observables(), 2);

            observable.set(10);

            // Value updated
            assert_eq!(*observable.peek(), 10);

            // Both updated
            assert_eq!(observer.updates.borrow().len(), 1);
            assert_eq!(*observer.updates.borrow(), *observer_2.updates.borrow());
            assert_eq!(*observer.updates.borrow(), [10]);
        }

        observable.mutate(|n| n * 2);

        // Value updated
        assert_eq!(*observable.peek(), 20);

        // Dead reference pruned
        assert_eq!(observable.observables(), 1);

        // Observer updated
        assert_eq!(*observer.updates.borrow(), [10, 20]);
    }
}