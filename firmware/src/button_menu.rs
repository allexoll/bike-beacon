use crate::ButtonEvent;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ClickEvent {
    Short,
    Long,
    VeryLong,
}

pub struct ButtonMenu {
    state: ButtonEvent,
    last_event_counter: u32,
}

impl ButtonMenu {
    pub fn new() -> Self {
        Self {
            state: ButtonEvent::Released,
            last_event_counter: 0,
        }
    }

    // tick
    pub fn tick(&mut self) -> Option<()> {
        // if the button is pressed, we increment the counter
        if self.state == ButtonEvent::Pressed {
            self.last_event_counter += 1;
        }
        // return Some on the tick between different intervals
        match self.last_event_counter {
            20 | 50 => Some(()),
            _ => None,
        }
    }

    /// process a button event.
    /// store the menu state, and return the click event
    /// short click: between 2 and 20 ticks.
    /// long click: between 20 and 50 ticks.
    /// very long click: more than 50 ticks long.
    /// return the event, or None if no event.
    pub fn process_event(&mut self, ev: ButtonEvent) -> Option<ClickEvent> {
        // if released, return the appropriate event, and reset the counter
        if ev == ButtonEvent::Released {
            let ev = match self.last_event_counter {
                0..=1 => None,
                2..=20 => Some(ClickEvent::Short),
                21..=50 => Some(ClickEvent::Long),
                _ => Some(ClickEvent::VeryLong),
            };
            self.last_event_counter = 0;
            ev
        } else {
            self.last_event_counter = 0;
            self.state = ev;
            None
        }
    }
}
