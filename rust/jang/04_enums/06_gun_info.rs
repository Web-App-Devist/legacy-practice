// Define an enum for the gun type
#[derive(Debug)]
enum GunType {
    Pistol,
    Rifle,
    Shotgun,
    Sniper,
}

// Define an enum for the extra modifications
#[derive(Debug)]
enum Extra {
    Silencer,
    Scope,
    ExtendedMags,
    None,
}

// Define the Gun struct
#[derive(Debug)]
struct Gun {
    gun_type: GunType,
    recoil_time: f32, // Assume recoil time is in seconds
    magazine_size: usize,
    extra: Extra,
}

// Implementation of methods for the Gun struct
impl Gun {
    // Constructor function to create a new Gun instance
    fn new(gun_type: GunType, recoil_time: f32, magazine_size: usize, extra: Extra) -> Gun {
        Gun {
            gun_type,
            recoil_time,
            magazine_size,
            extra,
        }
    }

    // Method to update the gun's extra modification
    fn update_extra(&mut self, new_extra: Extra) {
        self.extra = new_extra;
        // Adjust recoil time and magazine size based on the new extra
        match self.extra {
            Extra::Silencer => {
                self.recoil_time *= 0.9; // Example reduction in recoil time with a silencer
                self.magazine_size -= 1; // Example decrease in magazine size with a silencer
            }
            Extra::Scope => {
                self.recoil_time *= 1.1; // Example increase in recoil time with a scope
                                         // No change in magazine size with a scope
            }
            Extra::ExtendedMags => {
                // No change in recoil time with extended mags
                self.magazine_size += 5; // Example increase in magazine size with extended mags
            }
            Extra::None => {
                // No modifications for None extra
            }
        }
    }

    // Method to display information about the gun
    fn display_info(&self) {
        println!("Gun Type: {:?}", self.gun_type);
        println!("Recoil Time: {:.2} seconds", self.recoil_time);
        println!("Magazine Size: {}", self.magazine_size);
        println!("Extra: {:?}", self.extra);
    }
}

fn main() {
    // Create a new Gun instance
    let mut my_gun = Gun::new(GunType::Rifle, 1.0, 30, Extra::None);

    // Display initial information
    println!("Initial Gun Information:");
    my_gun.display_info();
    println!();

    // Simulate user input to change the extra modification
    println!("Changing Extra to Silencer...");
    my_gun.update_extra(Extra::Silencer);
    println!("Updated Gun Information:");
    my_gun.display_info();
    println!();

    // Simulate another change
    println!("Changing Extra to Scope...");
    my_gun.update_extra(Extra::Scope);
    println!("Updated Gun Information:");
    my_gun.display_info();
    println!();

    // Simulate another change
    println!("Changing Extra to Extended Mags...");
    my_gun.update_extra(Extra::ExtendedMags);
    println!("Updated Gun Information:");
    my_gun.display_info();
    println!();
}
