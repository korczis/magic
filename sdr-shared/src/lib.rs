#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    #[quickcheck]
    fn double_reversal_is_identity(a: i64, b: i64) -> bool {
        let c = a + b;
        c - b == a
    }
}
