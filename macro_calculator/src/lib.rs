pub use json::{object, JsonValue};
pub use json;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        // Extract calories from the kcal string (second element)
        let cals: f64 = food.calories[1]
            .trim_end_matches("kcal")
            .parse()
            .unwrap_or(0.0);

        // Multiply each value by number of portions
        total_cals += cals * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    // Round to 2 decimal places
    let round_to_2 = |x: f64| {
        let rounded = (x * 100.0).round() / 100.0;
        // If the number ends with .x0, convert to .x
        if rounded.fract() * 10.0 % 1.0 == 0.0 {
            (rounded * 10.0).round() / 10.0
        } else {
            rounded
        }
    };

    object! {
        "cals": round_to_2(total_cals),
        "carbs": round_to_2(total_carbs),
        "proteins": round_to_2(total_proteins),
        "fats": round_to_2(total_fats)
    }
}