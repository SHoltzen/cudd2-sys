#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    /// A simple test to see if two applications yield the same result
    #[test]
    fn test_cudd_simple() {
        unsafe {
            let m = Cudd_Init(10, 0, 0, 0, 0);
            let v1 = Cudd_bddNewVar(m);
            let v2 = Cudd_bddNewVar(m);
            let conj1 = Cudd_bddAnd(m, v1, v2);
            let conj2 = Cudd_bddAnd(m, v2, v1);
            assert!(conj1 == conj2);
            Cudd_Deref(v1);
            Cudd_Deref(v2);
            Cudd_Deref(conj1);
            Cudd_Deref(conj2);
            Cudd_Quit(m);
        }
    }
}
