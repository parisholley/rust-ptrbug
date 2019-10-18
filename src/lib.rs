#![feature(test)]

extern crate test;

use crate::bind::{startEngine, engineStruct, getWorker, releaseWorker};
use std::mem::MaybeUninit;
use std::ffi::CString;

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

static panic_static:&str = "this static str should not cause panic";

const panic_string:&str = "this const str should not cause panic";

fn should_not_cause_panic_println(){
    println!("this println does not cause a panic");
}

fn should_not_cause_panic_cstring(){
    let cstr = CString::new("foobar").expect("CString::new failed");
    let cstr_ptr = cstr.into_raw();
}

fn should_not_cause_panic_str(){
    "this inline str should not cause panic";
}

fn should_not_cause_panic_static_to_string(){
    panic_static.to_string();
}

fn should_not_cause_panic_const_to_string(){
    panic_string.to_string();
}

fn should_not_cause_panic_to_string(){
    "this inline str.to_string() should not cause panic".to_string();
}

impl Engine{
    pub fn new() -> Engine{
        let mut engine:MaybeUninit<engineStruct> = unsafe { MaybeUninit::uninit()};

        unsafe {
            let mut engine = MaybeUninit::uninit();

            startEngine(engine.as_mut_ptr());

            let mut assumed = engine.assume_init();

            should_not_cause_panic_println();
            should_not_cause_panic_cstring();
            should_not_cause_panic_to_string();
            should_not_cause_panic_str();
            should_not_cause_panic_const_to_string();
            should_not_cause_panic_static_to_string();

            do_get_release_worker(&mut assumed);

            should_not_cause_panic_println();
            should_not_cause_panic_cstring();
            should_not_cause_panic_to_string();
            should_not_cause_panic_str();
            should_not_cause_panic_const_to_string();
            should_not_cause_panic_static_to_string();

            do_get_release_worker(&mut assumed);

            should_not_cause_panic_println();
            should_not_cause_panic_cstring();
            should_not_cause_panic_to_string();
            should_not_cause_panic_str();
            should_not_cause_panic_const_to_string();
            should_not_cause_panic_static_to_string();

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
    use std::ffi::CString;

    #[test]
    fn no_panic() {
        let mut engine = Engine::new();

        engine.get_release_worker();
        engine.get_release_worker();
    }

    #[test]
    fn no_panic_str() {
        let mut engine = Engine::new();

        engine.get_release_worker();

        should_not_cause_panic_str();

        engine.get_release_worker();
    }

    #[test]
    fn panic_static_to_string() {
        let mut engine = Engine::new();

        engine.get_release_worker();

        should_not_cause_panic_static_to_string();

        engine.get_release_worker();
    }

    #[test]
    fn panic_const_to_string() {
        let mut engine = Engine::new();

        engine.get_release_worker();

        should_not_cause_panic_const_to_string();

        engine.get_release_worker();
    }

    #[test]
    fn panic_tostring() {
        let mut engine = Engine::new();

        engine.get_release_worker();

        should_not_cause_panic_to_string();

        engine.get_release_worker();
    }

    #[test]
    fn panic_cstring() {
        let mut engine = Engine::new();

        engine.get_release_worker();

        should_not_cause_panic_cstring();

        engine.get_release_worker();
    }

    #[test]
    fn panic_println() {
        let mut engine = Engine::new();

        engine.get_release_worker();

        should_not_cause_panic_println();

        engine.get_release_worker();
    }
}