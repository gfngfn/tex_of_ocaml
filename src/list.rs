#[derive(Debug)]
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

    pub fn singleton(x: T) -> List<T> {
        List::Cons(x, Box::new(List::Nil))
    }
}

pub fn append<T>(xs: List<T>, ys: List<T>) -> List<T> {
    match xs {
        List::Nil => ys,
        List::Cons(x, xtail) => List::Cons(x, Box::new(append(*xtail, ys))),
    }
}
