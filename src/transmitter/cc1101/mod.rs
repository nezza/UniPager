use config::Config;
use transmitter::Transmitter;
use std::vec;
pub struct CC1101Transmitter;

impl CC1101Transmitter {
    pub fn new(_: &Config) -> CC1101Transmitter {
        warn!("Using CC1101 transmitter");
        CC1101Transmitter {}
    }
}

impl Transmitter for CC1101Transmitter {
    fn send(&mut self, gen: &mut Iterator<Item = u32>) {
        let mut vector: Vec<u8> = vec![0;0];
        // First we need to convert the whole stuff to u8
        for word in gen {
            vector.push((word >> 24 & 0xFF) as u8);
            vector.push((word >> 16 & 0xFF) as u8);
            vector.push((word >> 8 & 0xFF) as u8);

            vector.push((word & 0xFF) as u8);
            info!("CC1101: {:x}", word);
        }

        // We have bytes now, ready to be transmitted.

        // Transmission happens like this:
        // - Get length of transmission
        // - set inifinte packet length (LENGTH_CONFIG=2)
        // - set PKTLEN to transmission length % 256
        // - fill fifo with 64 bytes
        // - start transmission
        // - keep filling fifo with 32 bytes as soon as it's filled less than 32
        // - as soon as less than 256 bytes remain to be send, set LENGTH_CONFIG=0


        // for byte in vector {
        //  info!("BYTE: {:x}", byte);
        // }



        let mut transmitted = 0;
        let length = vector.len();
        let length_mod = length % 256;

        // set LENGTH_CONFIG to 2
        // set PKTLEN to length_mod
        {
            let fifo_data: &[u8] = &vector[0..64];
            transmitted += 64;
            info!("Transmitting first 64 bytes {:x?}", &fifo_data);
        }
        // set fifo..

        let mut pkt_length_active = false;

        while(transmitted < length) {
            // while(GET_FIFO_SIZE > 31) {

            // }
            // set fifo
            if(length - transmitted > 64) {
                let tx_data = &vector[transmitted..transmitted+64];
                transmitted += 64;
                info!("Transmitting {:x?}", &tx_data);
            } else {
                let remaining = length - transmitted;
                let tx_data = &vector[transmitted..transmitted+remaining];
                transmitted += remaining;
                info!("Transmitting {:x?}", &tx_data);
            }

            if(!pkt_length_active && length - transmitted < 256) {
                info!("ACTIVATING LIMIT!");
                pkt_length_active = true;
            }
        }
        info!("TRANSMITTED");

    }
}
