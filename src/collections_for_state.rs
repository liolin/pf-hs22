struct Workpiece {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

impl Workpiece {
    pub fn new() -> Self {
        Workpiece {
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    pub fn increment_minute(&mut self) {
        self.minutes = (self.minutes + 1) % 24;
    }

    pub fn increment_hour(&mut self) {
        self.hours = (self.hours + 1) % 24;
    }

    pub fn tick(&mut self) {
        self.seconds += 1;
        if self.seconds == 60 {
            self.seconds = 0;
            self.minutes += 1;
            if self.minutes == 60 {
                self.minutes = 0;
                self.hours = (self.hours + 1) % 24;
            }
        }
    }

    pub fn display(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        out.write(format!("{}:{}:{}\n", self.hours, self.minutes, self.seconds).as_bytes())?;
        Ok(())
    }
}

pub struct StateMachine {
    displaying_time: Vec<Workpiece>,
    setting_hours: Vec<Workpiece>,
    setting_minutes: Vec<Workpiece>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            displaying_time: vec![Workpiece::new()],
            setting_hours: vec![],
            setting_minutes: vec![],
        }
    }
}

impl crate::StateMachine for StateMachine {
    fn display(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        // TODO: Fix unwrap
        self.displaying_time
            .iter()
            .for_each(|w| w.display(out).unwrap());
        self.setting_hours
            .iter()
            .for_each(|w| w.display(out).unwrap());
        self.setting_minutes
            .iter()
            .for_each(|w| w.display(out).unwrap());
        Ok(())
    }

    fn tick(&mut self) {
        self.displaying_time.iter_mut().for_each(|w| w.tick());
    }

    fn increment(&mut self) {
        self.setting_hours
            .iter_mut()
            .for_each(|w| w.increment_hour());
        self.setting_minutes
            .iter_mut()
            .for_each(|w| w.increment_minute());
    }

    fn change_mode(&mut self) {
	std::mem::swap(&mut self.setting_hours, &mut self.displaying_time);
	std::mem::swap(&mut self.setting_minutes, &mut self.displaying_time);
    }
}
