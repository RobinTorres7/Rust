fn main() {

let numero_1 = 123;
    let numero_2 = 321;

    let suma = numero_1 + numero_2;
    
    loop {
        println!("por favor escribir la suma de {} y {}: ", numero_1, numero_2);

        let mut suma_usuario = String::new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();
        let suma_usuario_int : i32 = suma_usuario.trim().parse().unwrap();
    
        if suma_usuario_int == suma{
            println!("Excelente, el resultado {} es correcto", suma);
            break;
        } else {
            println!("El resultado {} no es correcto por favor intentalo de nuevo", suma_usuario_int);
        }
    
    }*/

}