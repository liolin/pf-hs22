mod method_for_states;


fn main() {
    run_method_for_states();
}

fn run_method_for_states() {
    let mut st = method_for_states::StateMachine::new();
    st.display(&mut std::io::stdout().lock()).unwrap();

    println!("TICK");
    st.tick();
    st.display(&mut std::io::stdout().lock()).unwrap();

    println!("INCREMENT");
    st.increment();
    st.display(&mut std::io::stdout().lock()).unwrap();

    println!("MODE");
    st.change_mode();
    st.display(&mut std::io::stdout().lock()).unwrap();

    println!("TICK");
    st.tick();
    st.display(&mut std::io::stdout().lock()).unwrap();

    println!("INCREMENT");
    st.increment();
    st.display(&mut std::io::stdout().lock()).unwrap();

    println!("MODE");
    st.change_mode();
    st.display(&mut std::io::stdout().lock()).unwrap();

    println!("TICK");
    st.tick();
    st.display(&mut std::io::stdout().lock()).unwrap();

    println!("INCREMENT");
    st.increment();
    st.display(&mut std::io::stdout().lock()).unwrap();

    println!("MODE");
    st.change_mode();
    st.display(&mut std::io::stdout().lock()).unwrap();
}
