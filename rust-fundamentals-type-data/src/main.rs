// use std::string;

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

    // TIPOS DE DATOS FLOTANTES
    // f32: 32 bits
    // f64: 64 bits

    // TIPOS DE DATOS BOOLEANOS
    // bool: true o false

    // TIPOS DE DATOS CARACTERES
    //- `String`:
    //- **Descripción**: Tipo de dato que representa una cadena de texto que es mutable y se almacena en el heap.
    //- **Uso**: Se utiliza cuando necesitas una cadena de texto que puede ser modificada o cuyo tamaño puede cambiar dinámicamente.

    //- `&str`:
    //- **Descripción**: Tipo de dato que representa una cadena de texto que es inmutable y se almacena en el stack.
    //- **Uso**: Se utiliza cuando necesitas una cadena de texto que no va a cambiar y cuyo tamaño es fijo.

    // TIPOS DE VARIABLES EN RUST
    // `let`
    // - **Descripción**: Se utiliza para declarar una variable.
    // - **Uso**: Se utiliza para declarar una variable inmutable por defecto. Es decir, una vez que se asigna un valor a una variable declarada con `let`, ese valor no puede ser modificado a menos que la variable sea mutable.
    // - **Ejemplo**:
    //     ```rust
    //     let x = 5; // `x` es una variable inmutable con el valor 5.
    //     ```
    // - **Nota**: Si necesitas que una variable pueda ser modificada después de su declaración, debes usar la palabra reservada `mut`.
    //     ```rust
    //     let mut x = 5; // `x` es una variable mutable con el valor 5.
    //     x = 10; // Ahora `x` tiene el valor 10.
    //     ```

    // `let mut`
    // - **Descripción**: Se utiliza para declarar una variable mutable.
    // - **Uso**: Permite modificar el valor de la variable después de su declaración. Esto significa que puedes reasignar nuevos valores a la variable en cualquier momento.
    // - **Ejemplo**:
    //     ```rust
    //     let mut x = 5; // `x` es una variable mutable con el valor 5.
    //     x = 10; // Ahora `x` tiene el valor 10.
    //     ```

    // `const`
    // - **Descripción**: Se utiliza para declarar una constante que debe ser inmutable y cuyo valor debe ser conocido en tiempo de compilación.
    // - **Uso**: Se utiliza para declarar valores constantes que no cambiarán a lo largo de la ejecución del programa. Las constantes tienen que ser anotadas con un tipo explícito y su valor debe ser una expresión constante.
    // - **Nota**: Por convención, los nombres de las constantes se escriben en mayúsculas y con guiones bajos para separar las palabras y en las const no se infiere el tipo de dato que sera.
    // - **Ejemplo**:
    //     ```rust
    //     const PI: f32 = 3.14159; // `PI` es una constante de tipo f32 con el valor 3.14159.
    //     ```

    // CONTROL DE FLUJO EN RUST
    // `if`
    // - **Descripción**: Se utiliza para ejecutar un bloque de código si una condición es verdadera.
    // - **Uso**: Se utiliza para tomar decisiones basadas en el valor de una expresión booleana.
    // - **Ejemplo**:
    //     ```rust
    //     let x: i8 = 5;
    //     if x > 0 {
    //         println!("x es mayor que 0");
    //     }

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
    let float: f32 = 3.14;
    println!("El valor de float es: {}", float); // Imprime: El valor de float es: 3.14

    const TEST: &str = "Hola Mundo"; // constante de tipo &str
    println!("El valor de TEST es: {}", TEST); // Imprime: El valor de TEST es: Hola Mundo

    let x: i8 = 5;
    if x > 0 {
        println!("x es mayor que 0");
    } else {
        println!("x es menor que 0");
    }

    let y: i8 = 100;

    if y > 1 && y == x {
        println!("y es mayor que 1, y es igual a x");
    } else {
        println!("y no es mayor que 1, y no es igual a x");
    }

    ////////////////////////////////////////////////////////////////////////////////////////////
}
