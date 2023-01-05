use std::io::Write;

mod method_for_state;
mod collections_for_state;

trait StateMachine {
    fn display(&self, out: &mut impl Write) -> std::io::Result<()>;
    fn tick(&mut self);
    fn increment(&mut self);
    fn change_mode(&mut self);
}


fn main() {
    println!("METHOD FOR STATE");
    run_state_machine(&mut method_for_state::StateMachine::new());

    println!("\nCOLLECTIONS FOR STATE");
    run_state_machine(&mut collections_for_state::StateMachine::new())
}

fn run_state_machine(st: &mut impl StateMachine) {
    let mut out = std::io::stdout().lock();
    st.display(&mut out).unwrap();

    println!("TICK");
    st.tick();
    st.display(&mut out).unwrap();

    println!("INCREMENT");
    st.increment();
    st.display(&mut out).unwrap();

    println!("MODE");
    st.change_mode();
    st.display(&mut out).unwrap();

    println!("TICK");
    st.tick();
    st.display(&mut out).unwrap();

    println!("INCREMENT");
    st.increment();
    st.display(&mut out).unwrap();

    println!("MODE");
    st.change_mode();
    st.display(&mut out).unwrap();

    println!("TICK");
    st.tick();
    st.display(&mut out).unwrap();

    println!("INCREMENT");
    st.increment();
    st.display(&mut out).unwrap();

    println!("MODE");
    st.change_mode();
    st.display(&mut out).unwrap();
}
