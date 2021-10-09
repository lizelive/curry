mod tuple;

extern crate proc_macro;
extern crate quote;

macro_rules! reverse_applied {
    ($f:ident($($x:expr),+)) => {
        move |x1| Box::new(move |x2| $f(x1, x2))
    };
}

// macro_rules! reverse_applied_n {
//     ($f:ident) => {
//         move |x1|{
//             Box::new(
//                 move |x2| {
//                     $f(x1, x2)
//                 }
//             )
//         }
//     };
// }

// macro_rules! curry_applied{
//     ($f:ident, 0) => {
//         $f()
//     };
//     ($f:ident, 1) => {
//         move |x1| $f(x1)
//     };
//     ($f:ident, 2) => {
//         move |x1| Box::new(move |x2| $f(x1, x2))
//     };
//     ($f:ident, 3) => {
//         move |x1| Box::new( move |x2| Box::new(move |x3| $f(x1, x2, x3)))
//     };
// }

#[macro_use]
mod concat;

macro_rules! tuple_length {
    (()) => {
       0 
    };
    (($x:expr,)) => {
        1
     };
     (($x:expr,$($r:expr),*)) => {
        1 + tuple_length!{($($r:expr),*)}
     };
}
const LEN : usize = tuple_length!((1,2,3));

#[macro_export]
macro_rules! concat_tuple {
    // ($o:expr, ()) => {
    //     $o
    // };
    ((), $o:expr) => {
        $o
    };
    (($($l:expr),+ $(,)?), ($($r:expr),* $(,)?)) => {
        ($($l),+, $($r),*)
    };
}

#[macro_export]
macro_rules! prepend_tuple {
    ($l:expr, ($($r:expr),* $(,)?)) => {
        ($l, $($r),*)
    };
}

#[macro_export]
macro_rules! construct {
    ($f:ident, ($($x:expr),* $(,)?)) => {
        $f($($x),*)
    };
}

#[macro_export]
macro_rules! curry_applied {
    ($f:ident, 0, $args:expr) => {
        construct!($f,$args)
    };
    ($f:ident, 1, $args:expr) => {
        move |x1| curry_applied!($f, 0, prepend_tuple!{x1, $args})
    };
    ($f:ident, 2, $args:expr) => {
        move |x1| Box::new(curry_applied!($f, 1, prepend_tuple!{x1, $args}))
    }; // ($f:ident, 3) => {
       //     move |x1| Box::new( move |x2| Box::new(move |x3| $f(x1, x2, x3)))
       // };
       // ($f:ident, $n:literal) =>{
       //     curry_applied!($f in symbol_tuple_range!($n))
       // };
       // ($f:ident in $args:expr) =>{
       //     curry_applied!($f for $args in $args)
       // };
       // ($f:ident for () in $args:ident) =>{
       //     $f($args)
       // };
       // ($f:ident for ($x:ident) in $args:expr) =>{
       //     move |$x| $f($args)
       // };
       // ($f:ident for ($x:ident, $($r:ident),*) in $args:expr) =>{
       //     move |$x| Box::new(curry_applied!($f for ($($r),*) in $args))
       // };
}

// macro_rules! symbol_tuple_range {
//     (0) => {
//         ()
//     };
//     (1) => {
//         (x1,)
//     };
//     (2) => {
//         (x1, x2)
//     };
//     (3) => {
//         (x1, x2, x3)
//     };
//     (4) => {
//         (x1, x2, x3, x4)
//     };
//     (5) => {
//         (x1, x2, x3, x4, x5)
//     };
//     (6) => {
//         (x1, x2, x3, x4, x5, x6)
//     };
//     (7) => {
//         (x1, x2, x3, x4, x5, x6, x7)
//     };
// }

#[cfg(test)]
mod tests {
    //use list::symbol_tuple_range;

    use list::symbol_tuple_range;

    use crate::tuple::*;

    fn add<T>(a: T, b: T) -> T
    where
        T: std::ops::Add<Output = T>,
    {
        a.add(b)
    }

    fn double(a: isize) -> isize
    where
    {
        a+a
    }

    #[test]
    fn test_curry_applied() {

        let f = curry_applied!(add, 2, ());
        let g = f(1);
        let g = g(1);
        let g = g(1);
        
        let x = concat!("{}",1);
        assert_eq!(g(0), 1);
        assert_eq!(g(1), 2);
    }

    #[test]
    fn test_range() {
        assert_eq!((1..2).len(), 1);
        let (x1, x2) = (1, 2);
        let f = prepend_tuple!{x1,()};
        assert_eq!(symbol_tuple_range!(1, x), (x1,));
        assert_eq!(symbol_tuple_range!(2, x), couple(x1, x2));
        assert_eq!((x1,), monuple(x1));
    }
    #[test]
    fn it_works() {}
}
