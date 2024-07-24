
## 00-COMENTAR CODIGO

# Comentarios regulares que son ignorados por el compilador:
// Comentarios de línea que llegan hasta el final de la línea.
/* Comentarios de bloque que se extienden hasta el delimitador de cierre. */

# Comentarios de documentación que se analizan en la documentación HTML de la biblioteca:
/// Genera documentación de la biblioteca para el siguiente elemento.
//! Genera documentación de la biblioteca para el elemento contenedor.


## 01-Tipos de datos y variables

-----------------------------------------------------------------------------
Nota: Cuales son los tipos de datos. Son primitivos y compuestos 

# Primitivos
    let a: i8 = -128; // 1 byte
    let b: u8 = 255; // 1 byte
    let c: i32 = -2147483648; // 4 bytes
    let x: f32 = 3.14; // 4 bytes
    let z: char = 'z'; // 4 bytes
    let t: bool = true; // 1 byte

# Compuestos
    let tuple: (i32, f64, char) = (42, 3.14, 'a'); // 4 + 8 + 4 = 16 bytes
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // 4 * 5 = 20 bytes
    let slice: &[i32] = &array[1..3]; // 2 * 4 = 8 bytes (más la referencia)
------------------------------------------------------------------------------


# VARIABLES

- let con esto podemos crear variables con el cual le guardamos cosas que queramos pueden ser string,numeros,booleanos, esto no puede mutar. a menos que se lo indiques usando mut

-------------------------------------------
Nota: si usamos &str o String. hay diferencias en cuanto su uso. Ambos son cadenas de texto
solamente se pueden diferenciar que &str es inmutable mientras que las cadenas de texto String son mutables
-------------------------------------------