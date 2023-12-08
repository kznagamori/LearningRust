// 基本となる構造体
struct Animal {
    name: String,
    noise: String,
}

impl Animal {
    fn new(name: String, noise: String) -> Animal {
        Animal { name, noise }
    }

    fn make_noise(&self) -> String {
        self.noise.clone()
    }
}

// Dog 構造体
struct Dog {
    animal: Animal,
}

impl Dog {
    fn new() -> Dog {
        Dog {
            animal: Animal::new("Dog".to_string(), "Bark!".to_string()),
        }
    }

    fn make_noise(&self) -> String {
        self.animal.make_noise()
    }
}

// Cat 構造体
struct Cat {
    animal: Animal,
}

impl Cat {
    fn new() -> Cat {
        Cat {
            animal: Animal::new("Cat".to_string(), "Meow!".to_string()),
        }
    }

    fn make_noise(&self) -> String {
        self.animal.make_noise()
    }
}

fn main() {
    let dog = Dog::new();
    let cat = Cat::new();

    println!("Dog says: {}", dog.make_noise());
    println!("Cat says: {}", cat.make_noise());
}
