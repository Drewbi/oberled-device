// use volatile::VolatileRef;

use crate::screen::Screen;

pub mod test;

pub use test::Test;

pub enum ModeType {
    Test(Test),
}

impl PartialEq for ModeType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ModeType::Test(_), ModeType::Test(_)) => true,
        }
    }
}

impl Eq for ModeType {}

pub trait ModeTrait {
    fn activate(&mut self, screen: &mut Screen);
    fn update(&mut self, screen: &mut Screen);
}

pub struct Modes {
    current_mode: ModeType,
}

impl Modes {
    pub fn new(screen: &mut Screen) -> Self {
        let mut mode = Test::new();
        mode.activate(screen);

        Self {
            current_mode: ModeType::Test(mode),
        }
    }

    // pub fn set_mode(&mut self, new_mode: &mut ModeType, screen: &mut Screen) {
    //     match new_mode {
    //         ModeType::Test(mode) => mode.activate(screen),
    //     }
    //     self.current_mode = new_mode;
    // }

    pub fn update(&mut self, screen: &mut Screen) {
        // match &self.current_mode {
        //     ModeType::Test(mode) => mode.update(screen),
        // }
        if let ModeType::Test(test) = &mut self.current_mode {
            test.update(screen)
        }
    }
}