pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    pub fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, ref tail) => 1 + tail.len(),
        }
    }

    pub fn foldl<A>(self, f: fn(A, T) -> A, acc: A) -> A {
        match self {
            List::Nil => acc,
            List::Cons(x, tail) => tail.foldl(f, f(acc, x)),
        }
    }
}
