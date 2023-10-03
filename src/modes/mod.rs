// use volatile::VolatileRef;

use crate::screen::Screen;

pub mod test;
pub mod rando;

use hal::Rng;
pub use test::Test;
pub use rando::Rando;

pub enum ModeType<'a> {
    Test(Test),
    Rando(Rando<'a>),
}

impl PartialEq for ModeType<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ModeType::Test(_), ModeType::Test(_)) => true,
            (ModeType::Rando(_), ModeType::Rando(_)) => true,
            _ => false,
        }
    }
}

impl Eq for ModeType<'_> {}

pub trait ModeTrait {
    fn activate(&mut self, screen: &mut Screen);
    fn update(&mut self, screen: &mut Screen);
}

pub struct Modes<'a> {
    current_mode: ModeType<'a>,
}

impl<'a> Modes<'a> {
    pub fn new(screen: &mut Screen, rng: Rng<'a>) -> Self {
        let mut mode = Rando::new(rng);
        mode.activate(screen);

        Self {
            current_mode: ModeType::Rando(mode),
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
        if let ModeType::Rando(rando) = &mut self.current_mode {
            rando.update(screen)
        }
    }
}