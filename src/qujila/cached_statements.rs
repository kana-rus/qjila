pub(crate) static mut CACHED_STATEMENTS: private::CachedStatements = private::CachedStatements::init();

mod private {
    const CACHE_SIZE: usize = u32::MAX as usize;
    pub(crate) struct CachedStatements {
        statements: [Option<&'static str>; CACHE_SIZE],
    } impl CachedStatements {
        pub(super) const fn init() -> Self {
            CachedStatements { statements: [None; CACHE_SIZE] }
        }

        pub(crate) fn get<const key: super::super::cached::key>(
            &self
        ) -> Option<&'static str> {
            self.statements[key.0 as usize]
        }
        /// SAFETY: ensure that `CACHED_STATEMENTS.get::<key>()` returns `Some(_)`
        pub(crate) unsafe fn set<const key: super::super::cached::key>(
            &mut self,
            statement: &String,
        ) {
            let statement: &'static str = Box::leak(Box::new(statement.to_owned()));
            self.statements[key.0 as usize].replace(statement);
        }
    }
}
