fn split
fn main() {
    let input = "x = 1;
    y = 2;
    z = ---(x+y)*(x+-y);";

    if !input.contains(";") {
        println!("no semi colons :(");
    } else {
        let mut split = input.split(";");
        for assignment in split {
            println!("{}", assignment);
        }
    }
}
