#![feature(test)]
extern crate test;
extern crate parking_lot;

mod contend;
mod park_event;

use contend::{TestCase, contend};
use park_event::ParkEvent;

enum ParkEventTestCase {}
impl TestCase for ParkEventTestCase {
    type TestType = ParkEvent;

    fn create_value() -> ParkEvent {
        let l = ParkEvent::new();
        l.signal();
        return l;
    }
    fn do_stuff_with_value(value: &ParkEvent) {
        value.wait();
        value.signal();
    }
}

#[bench]
fn contend_lock_park_event(b: &mut test::Bencher) {
    contend::<ParkEventTestCase>(b);
}
