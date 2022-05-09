pub trait Get {
    fn get() -> Self;
}

pub trait GetRef<'a> {
    fn get_ref() -> &'a Self;
}
