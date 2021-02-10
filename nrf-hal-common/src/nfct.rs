use crate::pac::NFCT;

pub struct Nfct {
    p: NFCT,
}

impl Nfct {
    pub fn new(raw: NFCT, uicr: &Uicr) -> Self {
        Self { p: raw }
    }

    pub fn listen(&mut self, event: Event) {
        match event {
            Event::FieldDetected => {
                self.p.intenset.write(|w| w.fielddetected().set_bit());
            }
        }
    }

    pub fn ack_event(&mut self, event: Event) {
        match event {
            Event::FieldDetected => {
                self.p.events_fielddetected.reset();
            }
        }
    }
}

pub enum Event {
    FieldDetected,
}
