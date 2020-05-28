use std::{thread, time};

use crate::config::Config;
use crate::transmitter::Transmitter;


pub struct CC1101Transmitter;

impl CC1101Transmitter {
    pub fn new(_: &Config) -> CC1101Transmitter {
        warn!("Using CC101 transmitter");
        CC1101Transmitter {}
    }
}
impl Transmitter for CC1101Transmitter {
	fn send(&mut self, gen: &mut dyn Iterator<Item = u32>) {
		// info!{"CC1101 TRANSMITTER {}", gen.count()}
    for word in gen {
      info!("{:x}", word);
      for i in (0..32).rev() {
        
                // let bit = (word & (1 << i)) != 0;
                // self.atdata.set(bit);
      }
    }
		// for word in gen {
  //           info!("{:032b}", word);
  //           // count += 1;
  //       }
	}
}
