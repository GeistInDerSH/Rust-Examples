

pub fn print_bottles(bottle_count: u8)
{
    for i in (1..bottle_count).rev()
    {
        println!("{} bottles of beer on the wall. {} bottles of beer. Take one down, pass it around. {} bottles of beer on the wall.", i, i, i-1);
    }
}
