const PI:f32 = 3.14159;

fn main() {
    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    /* Mi primer hola mundo :D */
    println!("Holiwi de kiwi mundo! :D");

    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    /* Ejemplo con tipos de datos primitivos */
    //tipos_de_datos_primitivos();

    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    /* Ejemplo de uso de punteros */
    //ejemplo_punteros();

    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    /* Ejemplo de parseo en las variables */
    //ejemplo_parseo_variables();

    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    /* Ejemplo de funciones con retorno */
    //println!("El valor de retorno es: {}",suma(5,10));
    //let resultado = intercambiar(5, 10);
    //println!("Los valores 5,10 se retornan como: {} y {}", resultado.0, resultado.1);

    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    /* Ejemplo de bucles */
    //bucle_infinito();
    //bucle_white();
    //bucle_for();

    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    /* Utilizar un vector como ArrayList */
    //como_usar_listas();

    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    /* Estructura de control match, antes Switch(){case XXX: break; default: break;} */
    //estructura_match();

    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    /* Funciones inline para guardar su valor de retorno en una variable */
    //funciones_inline();
}


/* Funciones sin valores de retorno */
fn tipos_de_datos_primitivos() {
    let a:i32 = 42; // Tipo INT con numeros negativos
    let b:u8 = 100; // Tipo INT sin numeros negativos

    let c:f64 = 4.32313; // Tipo FLOAT | Por defecto es f32, double es f64

    let d:bool = true; // Tipo Boleano

    let e:&str = "Hola mundo desde un string :D"; // Tipo de dato string se declara como &str

    let f:[i8;4] = [10, 20, 30, 40]; // Los array se declaran let nombre:[tipo,cantidadElementos] = [n,n...n];

    let g:(u8,&str) = (1,"texto"); // Variable con 2 valores

    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    // Las constantes numericas no necesitan parseo al string
    println!("Tipo int32={} y tipo u8 sin negativos={}", a,b);
    println!("Tipo float={}",c);
    println!("Constante {}",PI);
    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    if (d) {
        println!("Si esto se pinta, la variable 'd' es true");
    }
    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    // no se puede imprimir la variable string "e" directamente, hay que pasar el formateo de dos modos
    // println!("Variable: {e}");
    // println!("Variable: {}", e);
    println!("Variable: '{e}'");

    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    // Recorriendo un Array
    println!("Imprimiendo un array:");
    for (indice, valor) in f.iter().enumerate() {
        print!("Índice:{} - Valor:{}  | ", indice, valor);
    }
    println!("");
    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    //Imprimiendo variable con 2 valores
    println!("Variable con 2 valores: valor1={} y valor2={}",g.0,g.1);

}

fn ejemplo_punteros(){
    // Declaración de una variable entera
    let mut numero:u8 = 22;
    // Creamos un puntero constante a nuestra variable entera
    let puntero_numero: *const u8 = &numero;
    // Declaración de una referencia inmutable
    let referencia_al_numero: u8 = numero;


    // Acceso a través del puntero
    unsafe {
        println!("Valor a través del puntero: {}", *puntero_numero);
    }
    // Acceso a través de la referencia
    println!("Valor a través de la referencia: {}", referencia_al_numero);

    println!("Modificamos nuestro valor origen :D");
    numero = 50;

    // Acceso a través del puntero
    unsafe {
        println!("Valor a través del puntero: {}", *puntero_numero);
    }
    // Acceso a través de la referencia
    println!("Valor a través de la referencia: {}", referencia_al_numero);
}

fn ejemplo_parseo_variables(){
    // Conversión mediante casting (palabra reservada 'as')
    let entero: i32 = 42;
    let flotante: f64 = entero as f64;
    println!("Entero: {}", entero);
    println!("Flotante después de casting: {}", flotante);


    // Conversión de punto decimal a entero
    let flotante_nuevo: f64 = 3.14;
    let entero_nuevo: i32 = flotante_nuevo as i32;
    println!("Flotante: {}", flotante_nuevo);
    println!("Entero después de casting: {}", entero_nuevo);

    println!("\n");

    // Conversión de cadena a entero
    let cadena_numero = "123";
    // Metodo .unwrap_or_else(|_| 0);
    // - Es como un try catch que captura el error en caso de ocurrir.
    // |_| Es un closure que toma un argumento, el _ simboliza que no lo usa y se ejecuta solo cuando de error
    // el 0 al final del parentesis simboliza el valor por defecto en caso de que la conversion falle
    let entero_desde_cadena: i32 = cadena_numero.parse().unwrap_or_else(|_| 0);
    println!("Cadena a entero: {}", entero_desde_cadena);

    // Probando el metodo .unwrap_or_else()
    let cadena_invalida:&str = "abc";
    // Intentar convertir la cadena a un entero
    let resultado_conversion_invalida: Result<i32, _> = cadena_invalida.parse();
    // Obtener el valor o ejecutar una acción en caso de error
    let valor_entero_invalido = resultado_conversion_invalida.unwrap_or_else(|_| {
        println!("Error al convertir la cadena a entero. Usando valor por defecto.");
        0
    });
    println!("Valor invalido: {}", valor_entero_invalido);
}

fn bucle_infinito(){
    let mut x:i16 = 0;
    loop {
        x += 1; // x++; no funciona
        if x == 20 {
            println!("La variable 'x' llego a {} en el bucle",x);
            break;
        }
    }
    println!("{}", x);
}
fn bucle_white(){
    let mut x:i8 = 0;
    while x != 20 { // se ejecutara mientras x sea diferente de 20
        x += 1;
    }
    println!("x es {}", x);
}
fn  bucle_for(){
    // El operador ".." crea numeros dentro de ese rango, pero no cuenta el numero final
    for x in 0..5 {
        print!("{}, ", x);
    }
    println!("\n");
    // El operador "..=" crea numeros dentro de ese rango, pero SI cuenta el numero final
    for x in 0..=5 {
        print!("{}, ", x);
    }
}
fn como_usar_listas(){
    // Crear un vector vacío con esta sintaxis
    // let nombreVariable:Vec<tipoDeDato> = Vec::new();
    let mut mi_lista: Vec<i32> = Vec::new();

    // Agregar elementos al vector
    mi_lista.push(1);
    mi_lista.push(2);
    mi_lista.push(3);
    mi_lista.push(4);
    mi_lista.push(5);

    // Acceder a elementos del vector
    println!("Segundo elemento: {}", mi_lista[1]);

    /* En Rust, Some es uno de los dos variantes del tipo de dato enum llamado Option.
       Option se utiliza para representar la posibilidad de que un valor esté presente
       (Some) o ausente (None). Es comúnmente utilizado para manejar casos en los que
       una operación puede fallar sin tener que recurrir a manejo de errores tradicional.*/
    // Eliminar el último elemento y obtener el valor eliminado
    if let Some(valor_eliminado) = mi_lista.pop() {
        println!("Valor eliminado: {}", valor_eliminado);
    } else {
        println!("La lista estaba vacía, no se eliminó ningún valor.");
    }

    // Iterar sobre el vector
    for elemento in mi_lista {
        println!("Elemento: {}", elemento);
    }
}

fn estructura_match(){
    let x:i32 = 5;

    // Solo aplica la primer condicion encontrada, si tenemos 2 o mas donde nuestro valor
    // califique, entonces solo sera valida la que este mas arriba

    match x {
        0 => {
            println!("encontré 0");
        }
        // podemos hacer comparaciones con múltiples valores
        1 | 5 => {
            println!("encontré 1 o 5!");
        }
        // podemos hacer comparaciones con iteradores
        3..=9 => {
            println!("encontré un número entre 3 y 9, ambos incluidos");
        }
        // podemos asignar el valor encontrado a una variable
        matched_num @ 10..=100 => {
            println!("encontré {}, un número entre 10 y 100!", matched_num);
        }

        // En caso de no cumplir ninguna condicion, el valor default de la estructura
        _ => {
            println!("No encontré ninguna condicion! >:D");
        }
    }
}
fn funciones_inline(){
    let x:i16 = 20;
    // Expresión ternaria en Rust, mejor conocido como if inline, delegado, etc.
    let operacion_ternaria:&str = if x > 18 { "Es mayor de edad" } else { "Es menor de edad" };
    println!("Resultado ternario: {}", operacion_ternaria);

    // funciones inline
    let operacion:i32 = {
        let x = 45;
        x * x
    };
    println!("Resultado= {}", operacion);
}



/* Funciones con valor de retorno */
// un solo retorno
fn suma(x:i32, y:i32) -> i32 {
    return x+y;
}
// multiples valores de retorno
fn intercambiar(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}