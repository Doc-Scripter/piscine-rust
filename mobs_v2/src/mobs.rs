pub mod boss;
pub mod member;

use boss::Boss;
use member::{Member, Role};

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub cities: Vec<(String, u8)>,
    pub members: Vec<Member>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        let new_member = Member::new(name, Role::Associate, age);
        self.members.push(new_member);
    }

    pub fn attack(&mut self, other_mob: &mut Mob) {
        // Calculate power combat scores
        let self_power = self.calculate_power();
        let other_power = other_mob.calculate_power();
        println!("self power: {}, other power: {}", self_power, other_power);
    
        // Determine the mob with the lower power combat score
        let (weaker_mob, stronger_mob) = if self_power <= other_power {
            (self, other_mob)
        } else {
            (other_mob, self)
        };
    
        // Remove the last member from the weaker mob
        if let Some(_weaker_member) = weaker_mob.members.pop() {
            println!(
                "Battle Result: {} defeated {}",
                stronger_mob.name, weaker_mob.name
            );
    
            // Transfer cities and wealth if the weaker mob has no members left
            if weaker_mob.members.is_empty() {
                stronger_mob.cities.extend(weaker_mob.cities.drain(..));
                stronger_mob.wealth += weaker_mob.wealth;
                println!("Winner's Cities: {:?}", stronger_mob.cities);
                println!("Winner's Wealth: {}", stronger_mob.wealth);
                weaker_mob.wealth = 0;
            }
    
            // Don't push the defeated member to the stronger mob, effectively removing them
        }
    }
    

    pub fn steal(&mut self, target: &mut Mob, amount: u32) {
        let stolen_amount = amount.min(target.wealth);
        self.wealth += stolen_amount;
        target.wealth -= stolen_amount;
    }

    pub fn conquer_city(&mut self, mobs:Vec<Mob>, city_name: String, value: u8) {
        let city_already_taken = mobs.iter().any(|mob| mob.cities.iter().any(|(name, _)| name == &city_name));
        if !city_already_taken {
            self.cities.push((city_name, value));
        }
    }

    fn calculate_power(&self) -> u32 {
        self.members.iter().map(|member| member.role_value()).sum()
    }
}