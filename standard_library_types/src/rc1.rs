// rc1.rs

// In this exercise, we want to express the concept of multiple owners via the Rc<T> type.
// This is a model of our solar system - there is a Sun type and multiple Planets.
// The Planets take ownership of the sun, indicating that they revolve around the sun.

use std::rc::Rc;

#[derive(Debug)]
struct Sun;

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Sun {
    fn count_info(&self, num: usize) {
        println!("Reference count = {}", num);
    }
}

impl Planet {
    fn details(&self) {
        println!("The {:#?}", self);
    }
}

pub fn run_rc1() {
    use Planet::*;

    let sun = Rc::new(Sun);
    sun.count_info(Rc::strong_count(&sun));

    let mercury = Mercury(Rc::clone(&sun));
    sun.count_info(Rc::strong_count(&sun));
    mercury.details();

    let venus = Venus(Rc::clone(&sun));
    sun.count_info(Rc::strong_count(&sun));
    venus.details();

    let earth = Earth(Rc::clone(&sun));
    sun.count_info(Rc::strong_count(&sun));
    earth.details();

    let mars = Mars(Rc::clone(&sun));
    sun.count_info(Rc::strong_count(&sun));
    mars.details();

    let jupiter = Jupiter(Rc::clone(&sun));
    sun.count_info(Rc::strong_count(&sun));
    jupiter.details();

    let saturn = Saturn(Rc::clone(&sun));
    sun.count_info(Rc::strong_count(&sun));
    saturn.details();

    let uranus = Uranus(Rc::clone(&sun));
    sun.count_info(Rc::strong_count(&sun));
    uranus.details();

    let neptune = Neptune(Rc::clone(&sun));
    sun.count_info(Rc::strong_count(&sun));
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    sun.count_info(Rc::strong_count(&sun));

    drop(uranus);
    sun.count_info(Rc::strong_count(&sun));

    drop(saturn);
    sun.count_info(Rc::strong_count(&sun));

    drop(jupiter);
    sun.count_info(Rc::strong_count(&sun));

    drop(mars);
    sun.count_info(Rc::strong_count(&sun));

    drop(earth);
    sun.count_info(Rc::strong_count(&sun));

    drop(venus);
    sun.count_info(Rc::strong_count(&sun));

    drop(mercury);
    sun.count_info(Rc::strong_count(&sun));

    assert_eq!(Rc::strong_count(&sun), 1);
}
