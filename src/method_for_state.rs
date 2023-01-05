use std::io::Write;

type Action = fn(&mut DataContext);
type ModeTransition = fn() -> Mode;

fn next_hour(data_context: &mut DataContext) {
    data_context.hours = (data_context.hours + 1) % 24;
}

fn next_minute(data_context: &mut DataContext) {
    data_context.minutes = (data_context.minutes + 1) % 60;
}

fn do_nothing(_: &mut DataContext) {}

fn update_time(data_context: &mut DataContext) {
    data_context.seconds += 1;
    if data_context.seconds == 60 {
        data_context.seconds = 0;
        data_context.minutes += 1;
        if data_context.minutes == 60 {
            data_context.minutes = 0;
            data_context.hours = (data_context.hours + 1) % 24;
        }
    }
}

fn next_mode_setting_hours() -> Mode {
    SETTING_HOURS
}

fn next_mode_setting_minutes() -> Mode {
    SETTING_MINUTES
}

fn next_mode_displaying_time() -> Mode {
    DISPLAYING_TIME
}

const DISPLAYING_TIME: Mode = Mode {
    increment: do_nothing,
    tick: update_time,
    change_mode: next_mode_setting_hours,
};

const SETTING_HOURS: Mode = Mode {
    increment: next_hour,
    tick: do_nothing,
    change_mode: next_mode_setting_minutes,
};

const SETTING_MINUTES: Mode = Mode {
    increment: next_minute,
    tick: do_nothing,
    change_mode: next_mode_displaying_time,
};

struct Mode {
    increment: Action,
    tick: Action,
    change_mode: ModeTransition,
}

struct DataContext {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

impl DataContext {
    pub fn new() -> Self {
        DataContext {
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

pub struct StateMachine {
    current_state: Mode,
    data_context: DataContext,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            current_state: DISPLAYING_TIME,
            data_context: DataContext::new(),
        }
    }
}

impl crate::StateMachine for StateMachine {
    fn display(&self, out: &mut impl Write) -> std::io::Result<()> {
        out.write(
            format!(
                "{}:{}:{}\n",
                self.data_context.hours, self.data_context.minutes, self.data_context.seconds
            )
            .as_bytes(),
        )?;

        Ok(())
    }

    fn tick(&mut self) {
        (self.current_state.tick)(&mut self.data_context);
    }

    fn increment(&mut self) {
	(self.current_state.increment)(&mut self.data_context);
    }

    fn change_mode(&mut self) {
        self.current_state = (self.current_state.change_mode)();
    }
}
