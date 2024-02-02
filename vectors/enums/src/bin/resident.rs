enum CampusAddress {
    Hostel(u8),
    Building(String),
}

struct CampusResident {
    name: String,
    address: CampusAddress,
}

fn main() {
    let c = CampusResident {
        name: String::from("John Doe"),
        address: CampusAddress::Hostel(16),
    };

    match c.address {
        CampusAddress::Hostel(n) => println!("{} lives in hostel {n}", c.name),
        CampusAddress::Building(s) => println!("{} lives in building {s}", c.name),
    }
}
