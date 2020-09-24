/// PythagTripple is a struct for holding the sides, a, b, & c of a triangle.
struct PythagTriple
{
    a: u32,
    b: u32,
    c: u32,
}

impl PythagTriple
{
    /// is_tripple checks to see if the values a, b, c
    /// form a pythagorean triple
    fn is_triple(&self) -> bool
    {
        let a2 = self.a.pow(2);
        let b2 = self.b.pow(2);
        let c2 = self.c.pow(2);

        return a2+b2 == c2;
    }
}

/// pythagorean_tripple is the entrypoint for this module. It reads in the users input and
/// returns if the given input is a valid pythagorean tripple or not.
pub fn pythagorean_triple()
{
    let trip = get_triple();

    let result = if trip.is_triple() { "" } else { "not " };
    println!("({}, {}, {}) is {}a pythagorean triple", trip.a, trip.b, trip.c, result);
}

/// get_tripple reads from stdin and forms a PythagTriple using the comma separated values
/// # Example
/// ```rust
/// let v: Vec<&str> = "3,4,5".split(",").collect();
/// let a: u32 = v[0].parse().unwrap();
/// let b: u32 = v[1].parse().unwrap();
/// let c: u32 = v[2].parse().unwrap();
/// assert_eq!(a, 3);
/// assert_eq!(b, 4);
/// assert_eq!(c, 5);
/// ```
fn get_triple() -> PythagTriple
{
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input.truncate(user_input.len() - 1);

    let v: Vec<&str> = user_input.split(",").collect();

    return PythagTriple{
        a: v[0].parse().unwrap(),
        b: v[1].parse().unwrap(),
        c: v[2].parse().unwrap(),
    };
}
