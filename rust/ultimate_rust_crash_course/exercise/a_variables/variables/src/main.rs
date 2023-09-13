const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    // let unised = 42;
    // READY_AMOUNT = 3;
    // let missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}
