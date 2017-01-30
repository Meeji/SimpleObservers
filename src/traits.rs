use std::rc::Weak;

/// Trait for a type with an observable value
///
/// Implementing this trait on a type allows it to be subscribed to by observers of type `O`
pub trait Observable<O: Observer<Observes = Self::Has>> {
    /// The type of the value of the `Observable`
    type Has: PartialEq;

    /// Registers an observer
    fn register(&mut self, observer: Weak<O>);

    /// Triggers an update for all registered observers
    ///
    /// This method sends the wrapped value to all observers.
    /// Ii should also clean up any dead `Weak` references the `Observable` is holding
    fn trigger(&mut self);

    /// Sets the value of the `Observable` without notifying subscribers
    fn set_silently(&mut self, data: Self::Has);

    /// Returns the value of the `Observable`
    fn peek(&self) -> &Self::Has;

    /// Sets the value of the `Observable` and notifies subscribers
    fn set(&mut self, data: Self::Has) {
        self.set_silently(data);
        self.trigger();
    }

    /// Sets the value of the `Observable` and notifies subscribers if that value changes
    fn set_if_changed(&mut self, data: Self::Has) {
        if data != *self.peek() {
            self.set(data)
        }
    }

    /// Mutates the value of the `Observable` and notifies subscribers
    ///
    /// # Examples
    /// ```
    /// # use simple_observables::structs::ObservableValue;
    /// # use simple_observables::traits::{Observable, Observer};
    /// # let mut observable = ObservableValue::<_, Box<Observer<Observes = _>>>::new(5usize);
    /// observable.set(5);
    /// observable.mutate(|n| n * 2);
    /// assert_eq!(*observable.peek(), 10);
    /// ```
    fn mutate<F>(&mut self, mut f: F)
        where F: FnMut(&Self::Has) -> Self::Has
    {
        let new_value = f(self.peek());
        self.set(new_value);
    }
}

/// Trait for a type that can subscribe to an `Observable`
///
/// Implementing this trait on a type allows it to subscribe to and observe an `Observable<Self, Observers=Self.Observes>`
pub trait Observer {
    /// The type of the value that the `Observable` is wrapping
    type Observes;

    /// Method that is called by the `Observable` when its value changes, or an update is triggered
    fn update(&self, data: &Self::Observes);
}

impl<T> Observer for Box<Observer<Observes = T>> {
    type Observes = T;

    fn update(&self, data: &Self::Observes) {
        (**self).update(data);
    }
}