const fn c() -> usize {42}

const fn f(i: usize) -> usize {
    i.pow(10).rem_euclid(31)
}

struct T1<const C: usize = 42>;
struct T2<const C: usize = 42>;
struct T3<const C: usize = 42>;

struct Table {
    id: T1::<{c()}>,
}
