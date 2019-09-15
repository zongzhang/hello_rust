//sound 模块
mod sound {
    fn guitar() {}

    pub mod instrument {
        pub fn clarinet() {
            println!("ss");
        }
    }
}

use crate::sound::instrument;

fn main() {
//    crate::sound::instrument::clarinet();
    instrument::clarinet();
}