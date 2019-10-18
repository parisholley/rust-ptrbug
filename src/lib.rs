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

fn do_get_release_worker(engine: &mut engineStruct) -> i32 {
    assert_engine(engine);

    let released = unsafe {
        let worker = getWorker(engine);

        releaseWorker(worker) as i32
    };

    assert_engine(engine);

    released
}

impl Engine{
    pub fn new() -> Engine{
        let mut engine:MaybeUninit<engineStruct> = unsafe { MaybeUninit::uninit()};

        unsafe {
            let mut engine = MaybeUninit::uninit();

            startEngine(engine.as_mut_ptr());

            let mut assumed = engine.assume_init();

            println!("this println does not cause a panic");

            do_get_release_worker(&mut assumed);
            do_get_release_worker(&mut assumed);

            Engine {
                engine: assumed
            }
        }
    }

    fn get_release_worker(&mut self) -> i32 {
        do_get_release_worker(&mut self.engine)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn panic() {
        let mut engine = Engine::new();

        engine.get_release_worker();

        println!("this println causes panic");

        engine.get_release_worker();
    }

    #[test]
    fn no_panic() {
        let mut engine = Engine::new();

        engine.get_release_worker();
        engine.get_release_worker();
    }
}