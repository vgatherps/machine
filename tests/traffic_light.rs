#[macro_use]
extern crate machine;

machine!(
    #[derive(Clone, Debug, PartialEq)]
    enum TrafficLight {
        Green { count: u8 },
        Orange,
        Red,
        BlinkingOrange,
    }
);

pub mod prefix {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Advance;
}

#[derive(Clone, Debug, PartialEq)]
pub struct PassCar {
    count: u8,
    name: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Toggle;

transitions!(TrafficLight,
  [
    (Green, prefix::Advance) => Orange,
    (Orange, prefix::Advance) => Red,
    (Red, prefix::Advance) => Green,
    (Green, PassCar) => [Green, Orange],
    (Green, Toggle) => BlinkingOrange,
    (Orange, Toggle) => BlinkingOrange,
    (Red, Toggle) => BlinkingOrange,
    (BlinkingOrange, Toggle) => Red
  ]
);

methods!(TrafficLight,
  [
    Green => get count: u8,
    Green => set count: u8,
    Green, Orange, Red, BlinkingOrange => default(false) fn working(&self) -> bool
  ]
);

impl Green {
    pub fn on_advance(self, _: prefix::Advance) -> Orange {
        Orange {}
    }

    pub fn on_pass_car(self, input: PassCar) -> GreenOnPassCar {
        let count = self.count + input.count;
        if count >= 10 {
            println!("reached max cars count: {}", count);
            GreenOnPassCar::Orange(TrafficLight::orange())
        } else {
            GreenOnPassCar::Green(Green { count: count })
        }
    }

    pub fn on_toggle(self, _: Toggle) -> BlinkingOrange {
        BlinkingOrange {}
    }

    pub fn working(&self) -> bool {
        true
    }
}

impl Orange {
    pub fn on_advance(self, _: prefix::Advance) -> Red {
        Red {}
    }

    pub fn on_toggle(self, _: Toggle) -> BlinkingOrange {
        BlinkingOrange {}
    }

    pub fn working(&self) -> bool {
        true
    }
}

impl Red {
    pub fn on_advance(self, _: prefix::Advance) -> Green {
        Green { count: 0 }
    }

    pub fn on_toggle(self, _: Toggle) -> BlinkingOrange {
        BlinkingOrange {}
    }

    pub fn working(&self) -> bool {
        true
    }
}

impl BlinkingOrange {
    pub fn on_toggle(self, _: Toggle) -> Red {
        Red {}
    }

    pub fn working(&self) -> bool {
        false
    }
}

#[test]
fn test() {
    use prefix::Advance;

    let mut t = TrafficLight::Green(Green { count: 0 });
    t = t.on_pass_car(PassCar {
        count: 1,
        name: "test",
    });
    t = t.on_pass_car(PassCar {
        count: 2,
        name: "test",
    });
    assert_eq!(t, TrafficLight::green(3).into());
    t = t.on_advance(Advance);
    //println!("trace: {}", t.print_trace());
    assert_eq!(t, TrafficLight::orange().into());

    t = t.on_advance(Advance);
    assert_eq!(t, TrafficLight::red().into());

    t = t.on_advance(Advance);
    assert_eq!(t, TrafficLight::green(0).into());
    t = t.on_pass_car(PassCar {
        count: 5,
        name: "test",
    });
    assert_eq!(t, TrafficLight::green(5).into());
    t = t.on_pass_car(PassCar {
        count: 7,
        name: "test",
    });
    assert_eq!(t, TrafficLight::orange().into());
    t = t.on_advance(Advance);
    assert_eq!(t, TrafficLight::red().into());
    t = t.on_pass_car(PassCar {
        count: 7,
        name: "test",
    });
    assert_eq!(t, TrafficLight::error().into());
    t = t.on_advance(Advance);
    assert_eq!(t, TrafficLight::error().into());
}
