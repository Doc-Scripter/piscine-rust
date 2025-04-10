use mobs_v2::*;

fn main() {
  let (mafia1, mafia2) = (
    Mob {
      name: "Hairy Giants".to_string(),
      boss: boss::Boss::new("Louie HaHa", 36),
      cities: vec![("San Francisco".to_string(), 7)],
      members: vec![
        member::Member::new("Benny Eggs", member::Role::Soldier, 28),
        member::Member::new("Jhonny", member::Role::Associate, 17),
        member::Member::new("Greasy Thumb", member::Role::Soldier, 30),
        member::Member::new("No Finger", member::Role::Caporegime, 32),
      ],
      wealth: 100000,
    },
    Mob {
      name: "Red Thorns".to_string(),
      boss: boss::Boss::new("Big Tuna", 30),
      cities: vec![("San Jose".to_string(), 5)],
      members: vec![
        member::Member::new("Knuckles", member::Role::Soldier, 25),
        member::Member::new("Baldy Dom", member::Role::Caporegime, 36),
        member::Member::new("Crazy Joe", member::Role::Underboss, 23),
      ],
      wealth: 70000,
    },
  );

  println!("{:?}\n{:?}", mafia1, mafia2);
}

/*
Mob { name: "Hairy Giants", boss: Boss { name: "Louie HaHa", age: 36 }, members: [Member { name: "Benny Eggs", role: Soldier, age: 28 }, Member { name: "Jhonny", role: Associate, age: 17 }, Member { name: "Greasy Thumb", role: Soldier, age: 30 }, Member { name: "No Finger", role: Caporegime, age: 32 }], cities: [("San Francisco", 7)], wealth: 100000 }
Mob { name: "Red Thorns", boss: Boss { name: "Big Tuna", age: 30 }, members: [Member { name: "Knuckles", role: Soldier, age: 25 }, Member { name: "Baldy Dom", role: Caporegime, age: 36 }, Member { name: "Crazy Joe", role: Underboss, age: 23 }], cities: [("San Jose", 5)], wealth: 70000 }

*/