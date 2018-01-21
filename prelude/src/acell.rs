use ::*;

pub struct ACell<T: Copy> {
    inner: Mutex<T>,
}

impl<T: Copy + Debug> Debug for ACell<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self.get())
    }
}

impl<T: Copy> ACell<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Mutex::new(value),
        }
    }
    pub fn get(&self) -> T {
        *self.inner.lock().unwrap()
    }
    pub fn set(&self, value: T) {
        *self.inner.lock().unwrap() = value;
    }
}