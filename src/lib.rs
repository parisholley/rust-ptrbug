#![feature(test)]

extern crate test;

use crate::bind::{startEngine, engineStruct, getWorker, releaseWorker};
use std::mem::MaybeUninit;

mod bind;

struct Engine{
    engine: engineStruct
}


fn assert_engine(engine:&engineStruct){
    unsafe{
        assert!(std::ptr::eq(engine.pool, (*(*engine.pool).engine).pool));
    }
}

impl Engine{
    pub fn new() -> Engine{
        let mut engine:MaybeUninit<engineStruct> = unsafe { MaybeUninit::uninit()};

        unsafe {
            let mut engine = MaybeUninit::uninit();

            startEngine(engine.as_mut_ptr());

            let worker = getWorker(engine.as_mut_ptr());

            releaseWorker(worker);

            let assumed = engine.assume_init();

            assert_engine(&assumed);

            Engine {
                engine: assumed
            }
        }
    }

    fn get_release_worker(&mut self) -> i32 {
        assert_engine(&self.engine);

        let released = unsafe {
            let worker = getWorker(&mut self.engine);

            releaseWorker(worker) as i32
        };

        assert_engine(&self.engine);

        released
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn panic() {
        let mut engine = Engine::new();

        println!("this println causes panic");

        let result = engine.get_release_worker();
    }

    #[test]
    fn no_panic() {
        let mut engine = Engine::new();

        let result = engine.get_release_worker();
    }
}