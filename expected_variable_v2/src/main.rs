use expected_variable_v2::*;

fn main() {
    println!(
        "{} close to it",
        expected_variable("On_Point", "on_point").unwrap()
    );
    println!(
        "{} close to it",
        expected_variable("soClose", "so_close").unwrap()
    );
    println!(
        "{:?}",
        expected_variable("something", "something_completely_different")
    );
    println!(
        "{} close to it",
        expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
    );
}
/*
100% close to it
88% close to it
None
67% close to it
*/