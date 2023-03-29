use std::{hash::Hasher, ops::BitXor};
use crate::{connection::Connection, error::Error};


/// A pretty qujila
pub struct Qujila(
    deadpool_postgres::Pool
); impl Qujila {
    pub(crate) async fn next(&self) -> Result<Connection, Error> {
        Ok(Connection(
            self.0.get().await?
        ))
    }
}

#[macro_export]
macro_rules! cached {
    ($q:ident) => {
        qujila::qujila::cache::<{
            qujila::qujila::key(file!(),line!(),column!())
        }>($q).await
    };
}

//#[cfg(test)]
#[allow(unused)] mod __sample__ {mod qujila {pub use crate::*;}
    use qujila::cached;
    async fn __sample__(q: &qujila::Qujila) {
        let cached_qujila = cached!(q);
    }
}




pub async fn cache<const KEY: cached::key>(q: &Qujila) -> cached::Qujila<KEY> {
    cached::Qujila(
        q.next().await.expect("Failed to get connection")
    )
}
pub const fn key(file: &'static str, line: u32, column: u32) -> cached::key {
    cached::key::init()
        .write_str(file)
        .write_u32(line)
        .write_u32(column)
}
mod cached {
    use std::ops::BitXor;
    use crate::connection::Connection;

    pub struct Qujila<const KEY: key>(
        pub(super) Connection
    );

    #[derive(PartialEq, Eq)]
    pub struct key(pub(in super) u32);
    impl key {
        pub(super) const fn init() -> Self {Self(0)}
        pub(super) const fn write_str(mut self, s: &'static str) -> Self {
            let mut bytes = s.as_bytes();
            while bytes.len() >= 4 {
                self = self.write_u32(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]));
                bytes = &bytes[4..];
            }
            while bytes.len() > 0 {
                self = self.write_u32(bytes[0] as u32);
                bytes = &bytes[1..]
            }
            self.write_u32(0xFF)
        }
        pub(super) const fn write_u32(self, i: u32) -> Self {
            Self(self.0
                .rotate_left(5)
                .bitxor(i)
                .wrapping_mul(0x27220A95)
            )
        }
    }
}

#[cfg(test)] mod test_cached {
    use super::cached::key;
    use std::ops::BitXor;

    fn fxhash_u32(hash: &mut u32, word: u32) {
        *hash = hash
            .rotate_left(5)
            .bitxor(word)
            .wrapping_mul(0x27220A95)
    }
    fn fxhash_str(s: &str) -> u32 {
        let mut hash: u32 = 0;
        let mut bytes = s.as_bytes();

        while bytes.len() >= 4 {
            fxhash_u32(&mut hash, u32::from_le_bytes(bytes[..4].try_into().unwrap()));
            bytes = &bytes[4..]
        }
        for b in bytes {
            fxhash_u32(&mut hash, *b as u32)
        }
        fxhash_u32(&mut hash, 0xFF);

        hash
    }

    #[test] fn check_str_hash() {
        for case in [
            "a",
            "abc",
            "hash",
            "fxhash",
            "src/main.rs",
        ] {
            assert_eq!(
                fxhash_str(case),
                key::init().write_str(case).0,
                "in {case:?}"
            );
        }
    }
}


/*
    pub fn split_array_mut<const N: usize>(&mut self) -> (&mut [T; N], &mut [T]) {
        let (a, b) = self.split_at_mut(N);
        // SAFETY: a points to [T; N]? Yes it's [T] of length N (checked by split_at_mut)
        unsafe { (&mut *(a.as_mut_ptr() as *mut [T; N]), b) }
    }
*/

/*
    pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
        Self::from_le(Self::from_ne_bytes(bytes))
    }

    pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
        // SAFETY: integers are plain old datatypes so we can always transmute to them
        unsafe { mem::transmute(bytes) }
    }

    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(not(target_endian = "little"))]
        {
            x.swap_bytes()
        }
    }

    pub const fn swap_bytes(self) -> Self {
        intrinsics::bswap(self as $ActualT) as Self
    }
*/

