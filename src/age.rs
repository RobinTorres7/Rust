fn main() {

println!("Introduce your name,please: ");

    let mut name : String  = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    name = name.trim().to_string();

   println!("Introduce your age,please: ");
    let mut age : String  = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    let age_int : u8 = age.trim().parse().unwrap();
    println!("Hi,Welcome  your age is {}",age);


    if age_int >=18 && age_int != 30 {
        println!("you can in into the sky")
    }
    else if age_int == 30{
println!("Sorry but you can't in, you have 30 years")
    }
    else {
        println!("oh sorry, you can't in. :( ")
    }
}