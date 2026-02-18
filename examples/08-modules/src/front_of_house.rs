// Module for front of house operations

pub mod hosting {
    pub fn add_to_waitlist() {
        println!("  Adding customer to waitlist");
    }
    
    pub fn seat_at_table() {
        println!("  Seating customer at table");
    }
}

pub mod serving {
    pub fn take_order() {
        println!("  Taking order");
    }
    
    pub fn serve_order() {
        println!("  Serving order");
    }
    
    pub fn take_payment() {
        println!("  Taking payment");
    }
}
