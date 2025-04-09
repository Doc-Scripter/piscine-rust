use error_types_v2::*;

fn main() {
    let mut form_output = Form {
        name: "Lee".to_owned(),
        password: "qwqwsa1dty_".to_owned(),
    };

    println!("{:?}", form_output);
    println!("{:?}", form_output.validate());

    form_output.name = "".to_owned();
    println!("{:?}", form_output.validate());

    form_output.name = "as".to_owned();
    form_output.password = "dty_1".to_owned();
    println!("{:?}", form_output.validate());

    form_output.password = "asdasASd(_".to_owned();
    println!("{:?}", form_output.validate());

    form_output.password = "asdasASd123SA".to_owned();
    println!("{:?}", form_output.validate());
}
/*
Form { name: "Lee", password: "qwqwsa1dty_" }
Ok(())
Err(FormError { form_values: ("name", ""), date: "2022-10-17 12:09:25", err: "Username is empty" })
Err(FormError { form_values: ("password", "dty_1"), date: "2022-10-17 12:09:25", err: "Password should be at least 8 characters long" })
Err(FormError { form_values: ("password", "asdasASd(_"), date: "2022-10-17 12:09:25", err: "Password should be a combination of ASCII numbers, letters and symbols" })
Err(FormError { form_values: ("password", "asdasASd123SA"), date: "2022-10-17 12:09:25", err: "Password should be a combination of ASCII numbers, letters and symbols" })

*/