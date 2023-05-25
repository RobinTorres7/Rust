fn main() {

    let mut names : Vec<String> = Vec::new();

    for i in 0..3{
        println!("Hi, Welcome, please, give me a name:");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    names.push(expression);
    }
    //este codigo es para acceder a los datos de mi array 
    for expression in names  {
        println!("The name is :{}",expression)
    }

}