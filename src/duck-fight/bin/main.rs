use rand::Rng;

// The struct for the duck itself. This is where you want to declare all of your variables/fields
struct Duck {
    health : i32,
    happiness: i32,
    power : i32,
    name : String
}
// The impl for the Duck struct, containing all of its methods.
impl Duck {
    // Constructor method in Rust
    fn new(hp : i32, h : i32, p : i32, n : &str) -> Self {
        Duck {
            health : hp,
            happiness : h,
            power : p,
            name : n.to_string()
        }
    }
    // Method in Rust where self will be altered
    fn take_damage(&mut self, damage : i32) {
        self.health -= damage;
    }
    // Method in Rust where self is referenced, but not changed
    fn attack(&self, other : &mut Duck) {
        let mut rng = rand::thread_rng(); // Happiness will randomly affect the damage to make it more fun
        let damage = self.power + rng.gen_range(0, self.happiness + 1);
        println!("{} attacks {} for {} damage!", self.name, other.name, damage);
        other.take_damage(damage);
    }
    // Implicit returns still work in a method
    fn is_dead(&self) -> bool { self.health <= 0 }
}

fn fight(duck_one : &mut Duck, duck_two : &mut Duck, round : i32) -> bool {
    println!("---------- ROUND {} ----------", round);
    let mut rng = rand::thread_rng();
    let coin_flip = rng.gen_range(0, 2); // See which duck should attack first
    if coin_flip == 0 {
        // Duck one attacks first
        duck_one.attack(duck_two);
        // Check if duck two is knocked out
        if !duck_two.is_dead() {
            duck_two.attack(duck_one);
            // Check if duck one is knocked out
            if duck_one.is_dead() {
                println!("{} is knocked out! {} wins the fight!", duck_one.name, duck_two.name);
                return true;
            }
        } else {
            println!("{} is knocked out! {} wins the fight!", duck_two.name, duck_one.name);
            return true;
        }
    } else {
        // Duck two attacks first
        duck_two.attack(duck_one);
        // Check if duck one is knocked out
        if !duck_one.is_dead() {
            duck_one.attack(duck_two);
            // Check if duck two is knocked out
            if duck_two.is_dead() {
                println!("{} is knocked out! {} wins the fight!", duck_two.name, duck_one.name);
                return true;
            }
        } else {
            println!("{} is knocked out! {} wins the fight!", duck_one.name, duck_two.name);
            return true;
        }
    }
    println!("End of round {}!\n\t{}: {}\n\t{}: {}", round, duck_one.name, duck_one.health, duck_two.name, duck_two.health);
    println!("--------------------");
    return false;
}

fn main() {
    let mut game_over = false;
    let mut d_o : Duck = Duck::new(100, 10, 1, "Jeremy"); // Instantiating a new Duck "object"
    let mut d_t : Duck = Duck::new(100, 10, 1, "Henry");

    println!("Hello, ladies and gentlemen! Today we're going to have an amazing game of Duck Fight!");
    println!("Two ducks will be facing against each other today: {} and {}!", d_o.name, d_t.name);
    println!("The last one standing wins!");

    let mut round = 1;
    let mut _input = String::new();
    let i = std::io::stdin();
    while !game_over {
        println!("(Press Enter To Continue)");
        let _bytes = i.read_line(&mut _input).unwrap();
        game_over = fight(&mut d_o, &mut d_t, round);
        round += 1;
    }
    println!("--------------------");
}

