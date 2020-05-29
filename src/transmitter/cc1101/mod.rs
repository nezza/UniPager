use config::Config;
use transmitter::Transmitter;

pub struct CC1101Transmitter;

impl CC1101Transmitter {
    pub fn new(_: &Config) -> CC1101Transmitter {
        warn!("Using CC1101 transmitter");
        CC1101Transmitter {}
    }
}

impl Transmitter for CC1101Transmitter {
    fn send(&mut self, gen: &mut Iterator<Item = u32>) {
        for word in gen {
            info!("CC1101: {:x}", word);
        }
    }
}
