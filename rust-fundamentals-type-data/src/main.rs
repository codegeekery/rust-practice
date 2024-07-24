use std::string;

fn main() {
   
    //  CREANDO COMENTARIOS
    ////////////////////////////////////////////////////////////////////////////////////////////
    //     (//) <-- Comentario de una sola linea
    //     (/* */) <-- Comentario de varias lineas
    ////////////////////////////////////////////////////////////////////////////////////////////

    // VARIABLES INMUTABLES (Lo dejare comentado para que no genere error. Pero puedes descomentar y probar ver que error te da)
    ////////////////////////////////////////////////////////////////////////////////////////////
    // let x = 5;
    // println!("El valor de x es: {}", x);
    // x = 10;
    // println!("El valor de x es: {}", x);
    // let string = "Esto es una cadena de texto que no se puede modificar";
    // string = "esto es un intento de modificar string";
    // println!("El resultado de la variable string es: {}", string);
    ////////////////////////////////////////////////////////////////////////////////////////////

    // VARIABLES MUTABLES
    ////////////////////////////////////////////////////////////////////////////////////////////

    // TIPOS DE DATOS NUMERICOS
    // Enteros con signo (pueden ser negativos o positivos)
    // i8: -128 a 127 (8 bits)
    // i16: -32,768 a 32,767 (16 bits)
    // i32: -2,147,483,648 a 2,147,483,647 (32 bits)
    // i64: -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807 (64 bits)
    // i128: -170,141,183,460,469,231,731,687,303,715,884,105,728 a 170,141,183,460,469,231,731,687,303,715,884,105,727 (128 bits)
    // isize: Depende del tamaño del puntero del sistema (32 bits en sistemas de 32 bits, 64 bits en sistemas de 64 bits)

    // Enteros sin signo (solo positivos)
    // u8: 0 a 255 (8 bits)
    // u16: 0 a 65,535 (16 bits)
    // u32: 0 a 4,294,967,295 (32 bits)
    // u64: 0 a 18,446,744,073,709,551,615 (64 bits)
    // u128: 0 a 340,282,366,920,938,463,463,374,607,431,768,211,455 (128 bits)
    // usize: Depende del tamaño del puntero del sistema (32 bits en sistemas de 32 bits, 64 bits en sistemas de 64 bits)
     let mut number: i8 = 120; // por defecto las variables son inmutables, para hacerlas mutables se le agrega la palabra reservada mut, ahora cada numero puedes inferirle un tamaño de bits, en este caso i8 es de 8 bits
     println!("El valor de number es: {}", number); // Imprime: El valor de number es: 5

     number = 10;
     println!("El valor de number es: {}", number); // Imprime: El valor de number es: 10

     let mut string = "Esto es una cadena de texto que se puede modificar"; // Tipo de dato &str este tipo de dato no se puede modificar a menos que le pongas mut de lo contrario veras un error si no pones mut
     println!("El valor de string es: {}", string); // Imprime: El valor de string es: Esto es una cadena de texto que se puede modificar
     string = "Modifique su valor";
     println!("El valor de string es: {}", string); // Imprime: El valor de string es: Modifique su valor

     let mut string = String::from("Esto es una cadena de texto que se puede modificar");
     println!("El valor de string es: {}", string); // Imprime: El valor de string es: Esto es una cadena de texto que se puede modificar
     string = String::from("Modifique su valor");
     println!("El valor de string es: {}", string); // Imprime: El valor de string es: Modifique su valor

    // TIPOS DE DATOS FLOTANTES
    // f32: 32 bits
    // f64: 64 bits
    let float: f32 = 3.14;
    println!("El valor de float es: {}", float); // Imprime: El valor de float es: 3.14
    


    ////////////////////////////////////////////////////////////////////////////////////////////
}