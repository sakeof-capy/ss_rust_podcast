use coinflip;

fn main() {
    let coinflip_result = if coinflip::flip() { "heads" } else { "tails" };
    println!("The coin lands on {}.", coinflip_result);
}