pub struct Defer {
    pub f: fn(),
}

impl Drop for Defer {
    fn drop(&mut self) {
        (self.f)();
        debug!("Dropped {:?}", self.f);
    }
}
