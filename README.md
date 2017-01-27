# SimpleObservers
A simple implementation of observables/observers in Rust


## Usage:
```rust
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

// Observer<Observes = T> trait is implemented for Box<Observer<Observes = T>>

fn main() {
    // ObservableValue implemented Observable<O> and wraps a value
    let mut observable = ObservableValue::new(5usize);

    // Observable stores weak references to observers
    let observer_one = Rc::new(SimpleObserver::new("obs1"));
    observable.register(Rc::downgrade(&observer_one));
    
    {
        let observer_two = Rc::new(SimpleObserver::new("obs2"));
        observable.register(Rc::downgrade(&observer_two));
        
        // Sets the value, prunes dead observable references and notifies the rest
        // Observables 1 and 2 update!
        observable.set(200);
    }
    
    // Mutate the inner value in place, notifying observers
    // Observable 2 reference is pruned, and observable 1 updates
    observable.mutate(|n| n + 5);

    // Observable can be updated without notifying observers
    observable.set_silent(1001);

    // Observable can be told to only update and notify if the new value is changing
    observable.set_if_changed(1001);

    // And you can notify the observers without updating the value
    observable.trigger();
}
```