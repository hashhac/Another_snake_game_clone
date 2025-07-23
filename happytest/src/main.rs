use crate::Access::Guest;
#[derive(Debug)]
    enum Access {
        Admin,
        Manager,
        User,
        Guest
    }
    struct User<'a> {
        name: &'a str,
        age: u8,
        money: u16,
        access : Access
    }
    fn main() {
        let mut user : User = User {name:"Jonathon", age:21, money:100,access:Guest};
        if !can_access(&user) {
            println!("You need to purchase access");
            println!("Access is $100 would you like to buy it? (y/n)");
            let mut access = String::new();
            std::io::stdin().read_line(&mut access).expect("Failed to read line");
            if access.trim().to_lowercase() == "y" {
                charge_user(&mut user);
                give_access(&mut user);
                }
                
            }
        }
    
    fn can_access(user : &User) -> bool {
        match user.access {
            Access::Admin => true,
            Access::Manager => true,
            Access::User => true,
            Access::Guest => false
        }
    }
    fn charge_user(user : &mut User<'_>) {
        if user.money >= 100 {
            println!("You have enough money and have been charged $100");
            user.money -= 100;
            println!("You now have ${}", user.money);
        } else {
            println!("You don't have enough money");
        }
    }
    fn give_access(user : &mut User<'_>) {
        user.access = Access::User;
    }