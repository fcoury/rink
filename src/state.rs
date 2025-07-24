pub struct State<T> {
    value: T,
    subscriber: Vec<Box<dyn Fn(&T)>>,
}

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        State {
            value,
            subscriber: Vec::new(),
        }
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        for callback in &self.subscriber {
            callback(&self.value);
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: Fn(&T) + 'static,
    {
        self.subscriber.push(Box::new(callback));
    }
}
