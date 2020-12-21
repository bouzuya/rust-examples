fn input1() -> std::io::Result<()> {
    // https://docs.rs/dialoguer/0.7.1/dialoguer/struct.Input.html#example-usage
    use dialoguer::Input;

    // Tea or coffee? [No]: Yes$
    let input: String = Input::new()
        .with_prompt("Tea or coffee?")
        .with_initial_text("Yes")
        .default("No".into())
        .interact_text()?;

    println!("{}", input);

    Ok(())
}

fn input2() -> std::io::Result<()> {
    // https://docs.rs/dialoguer/0.7.1/dialoguer/struct.Input.html#example-usage
    use dialoguer::Input;

    // : $
    let input = Input::<String>::new().interact_text()?;

    println!("{}", input);

    Ok(())
}

fn input3() -> std::io::Result<()> {
    use dialoguer::Input;

    // username: $
    let input = Input::<String>::new()
        .with_prompt("username")
        .interact_text()?;

    println!("{}", input);

    Ok(())
}

fn input4() -> std::io::Result<()> {
    // https://docs.rs/dialoguer/0.7.1/dialoguer/struct.Input.html#example
    use dialoguer::Input;

    // error: This is not a mail address
    // Enter email: $
    let mail: String = Input::new()
        .with_prompt("Enter email")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains('@') {
                Ok(())
            } else {
                Err("This is not a mail address")
            }
        })
        .interact()?;

    println!("{}", mail);

    Ok(())
}

fn main() -> std::io::Result<()> {
    input1()?;
    input2()?;
    input3()?;
    input4()?;
    Ok(())
}
