const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    // let test: i32 = 1;
    // READY_AMOUNT = 1;
    println!("Firing {} of my {} missiles...", ready, missiles);
    // missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}
