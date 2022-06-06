# Tabby

Repositorio para el Lenguaje Tabby, un lenguaje enfocado en estadística implementando en Rust, desarrollado para la clase de Compiladores en el Semestre Ene-Jun 2022

<p align="center"><img src="./tabby.png" width="300"/></p>

## Quick Reference 
Esta referencia está diseñada para dar un overview general de todo lo que se necesita saber para desarrollar programas en Tabby. Explica mediante ejemplos el uso y configuración del lenguaje, para que cualquiera pueda, después de leer esta guía, hacer su propio programa en Tabby y utilizarlo para lo que necesite. 

### Configuración del Ambiente
Para poder utilizar Tabby, lo único que se necesita es tener instalado Rust y Cargo. Para instalarlo en macOS, se puede utilizar el siguiente comando en la terminal 

> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


Si este comando no funciona, o si se quiere correr desde otro sistema operativo, se puede seguir el tutorial en la página oficial de Rust: https://www.rust-lang.org/learn/get-started

Para comprobar si se instalo correctamente, solo hace falta correr el siguiente comando:

> cargo --version


### Instalación de Tabby
Para instalar y obtener todo el código utilizado para compilar y ejecutar Tabby, se puede correr el siguiente comando en la terminal 

> git clone https://github.com/diegogrdc/Tabby.git

Después de la instalación, se debería de tener un folder llamado Tabby. Todos los comandos se corren desde la carpeta principal del repositorio, es decir, dentro de Tabby/.
Hello World
Como todo buen lenguaje de programación, comenzaremos a familiarizarnos con la sintaxis viendo cómo crear un programa en Tabby para imprimir “Hello World!” a la consola. 
```
Program helloWorld;

Tabby() {
    Write("Hello World!\n");
}
```

Listo! Este es el código para imprimir “Hello World!” a la consola. Vamos a analizar paso a paso para entender bien la sintaxis. 

Todos los programas en Tabby comienzan con una línea “Program id;”, en donde se elige el nombre del programa. Puede ser cualquiera, pero debe de seguir las reglas de identificadores en Tabby.

Además de esto, todos los programas contienen la función principal, llamada Tabby(). Esta función nunca recibe parámetros, y siempre tiene que ser declarada después de todas las otras funciones. La ejecución siempre comienza en esta función. 

Por último, dentro de los corchetes de la función, tenemos la instrucción Write, la cuál nos ayuda a imprimir a la consola. Dentro de los paréntesis se dan los elementos que se desean imprimir. En este caso, la literal “Hello World!\n”, con un salto de línea. 

Ahora, veamos cómo compilar y ejecutar este código   

### Compilar y Ejecutar

Ya con el ambiente configurado, es fácil compilar y ejecutar los programas. Todos los códigos que se quieran compilar y ejecutar deben de estar en la carpeta main/. Para el ejemplo, podemos agregar el código a un archivo llamado helloworld.tabby. 

Teniendo el archivo, corremos el siguiente comando 

> cargo run --bin compile helloworld


Esto mostrará en consola un mensaje de que se compiló correctamente, o que se encontró algún error léxico, sintáctico o semántico. Con el código anterior, obtenemos el siguiente resultado:


El programa fue ejecutado correctamente! Ahora, si vemos la carpeta, se creó un nuevo archivo llamado helloworld.tabbyic. Aquí se encuentra el código intermedio que se generó por la compilación. 

Finalmente, para ejecutar, correríamos el siguiente comando:


> cargo run --bin execute helloworld


Listo! Si corremos este comando, obtenemos el siguiente resultado:  


Vemos como se imprimió el “Hello World!” que habíamos esperado.
Estás listo para escribir, compilar y ejecutar tu primer programa en Tabby. Como reto, trata de crear un programa que imprima tu nombre a la consola.
### Variables
Para poder almacenar valores, y usarlos a través de diferentes partes del código, ya sea como lectura de la terminal (valores dados por el usuario), o generados por alguna expresión, es necesario almacenarlos en variables. Las variables son posiciones en la memoria que almacenan cierto tipo de datos, que podemos usar a través de nuestro programa. A continuación vemos un ejemplo de cómo podemos declarar variables en  Tabby:
```
Program variables;

Var Int a, b;
Var Float c;
Var Bool d;

Tabby() {
   Write(a, " ", b, " ", c, " ", d, "\n");
}
```

Lo primero que podemos ver, es que existen tres tipos de datos en Tabby:
Int, que almacena números enteros de 32 bits
Float, que almacena números flotantes de 64 bits 
Bool, que almacena valores de True o False.

La declaración de variables siempre va después del estatuto Program. 
Se pueden declarar varias variables por línea, separadas por comas, y la última por un punto y coma. Siempre llevan el prefijo Var, luego el tipo de dato, y finalmente los identificadores. Ahora, ese identificador se puede usar a través de todo el código para referenciar a ese valor o variable.
El código anterior declara las variables, y luego imprime sus valores. 
Si compilamos y ejecutamos  el siguiente código obtendremos la siguiente salida:

`0 0 0 False`


Vemos que todos los números se inicializan en 0, o False (que es equivalente a 0). 
Además, es importante mencionar que este tipo de variables se definen como Globales. Esto quiere decir que se pueden utilizar en cualquier parte del código. 

Más adelante veremos cómo declarar variables locales, y cómo asignar valores a estas y utilizarlas en estatutos más complejos. 

### Identificadores
Es importante mencionar, que los identificadores en Tabby siguen un cierto formato estricto. Todos los identificadores deben de comenzar con una letra minúscula, y después letras minúsculas y mayúsculas sin límite. Esto se debe a que las palabras reservadas en Tabby se escriben comenzando con una letra mayúscula, por lo que esta regla ayuda a diferenciar fácilmente los identificadores de las palabras reservadas. 

Por ejemplo, algunos identificadores válidos son: 
- var
- id
- otroID
- nombreMuyLargo 


Algunos inválidos son:
- Id (comienza con mayúscula)
- id1 (contiene números)
- i_d (contiene carácteres que no son letras)

El nombre del programa también es un identificador, por lo que debe de seguir estas reglas.
Además, los identificadores deben de ser únicos. La única excepción es con variables locales, pero esto se explicará más adelante. 
### Escritura
Como ya se pudo ver en códigos anteriores, la escritura se hace a través del estatuto Write(). Este puede recibir múltiples elementos para imprimir, separados por comas. Estos elementos pueden ser literales de texto, variables, llamadas a funciones, incluso expresiones. Por ejemplo, veamos el siguiente código: 
```
Program write;

Tabby() {
   Write(101, "\n");
   Write(2 * 3, "\n");
   Write(5 - 4, "\n");
   Write(2 / 2, "\n");
}
```

El código imprimirá:

```
101
6
1
1
```

Vemos que puede imprimir números y ecuaciones. Además, en el ejemplo de variables vimos que también imprime los valores de variables, o texto como “Hello World”. Es importante notar que no imprime automáticamente los saltos de línea, por lo que hay que usar el carácter \n para esto. 

### Lectura 
En cualquier lenguaje de programación es importante poder interactuar con el usuario a través de la consola. Imaginemos que un usuario quisiera saber cuánto es el cuadrado de un número. Para esto podemos crear un programa que lea un número, e imprima este número elevado al cuadrado. Veamos el siguiente código: 
```
Program squared;

Var Int n;

Tabby() {
   Read(n);
   Write(n * n, "\n");
}
```

Podemos ver que el estatuto de lectura es Read(). Este estatuto sólo tiene entre los paréntesis un único valor, el cuál es el nombre de una variable. El número se leerá de consola, y será almacenado en esa variable. Si en consola escribiéramos un 3, la variable n, después de la lectura, tendría el valor 3 hasta que se cambiara, o hasta el fin del programa. Read() es único para una variable, no se puede usar para leer dos o más variables a la vez.
Además, es importante notar que se lee un número por línea, por lo que intentar dar dos o más números en la misma línea de consola puede hacer que el programa tenga problemas de ejecución. 
Para el programa anterior, si el usuario diera 6 como valor en consola, obtendría 36 como resultado en consola. 
### Asignación
Hay veces en las que no solo queremos modificar el valor de una variable a través de lectura, sino haciendo alguna expresión o alguna lógica más compleja dentro del programa. Para esto, se utiliza la asignación, que se representa como un único símbolo de igual =. Esto hace que la variable a la izquierda almacene el valor de la expresión a la derecha. Por ejemplo: 
```
Program assign;

Var Int n;

Tabby() {
   n = 10 * 5 - 3;
}
```

En este programa, se evaluará la expresión, que resulta en este caso en 47, y este valor se almacena en n. Si hiciéramos un Write() de n, se imprimiría 47
La asignación solo se puede usar una vez por línea, por lo que cosas como a = b = 1, no son permitidas en Tabby. 
### Operaciones Aritméticas
Las operaciones aritméticas son 4 las operaciones principales para trabajar con números. Suma, Resta, División y Multiplicación. Estas se pueden utilizar con variables, números, incluso booleanos, siempre y cuando las operaciones tengan sentido con los tipos. Para ver qué tipo de operaciones son permitidas para qué tipo, ver la documentación. Estas ya fueron utilizadas en códigos anteriores, y  se ejemplifican más a fondo más adelante 
### Operaciones Condicionales
Las operaciones condicionales son las que hacen una comparación entre valores y regresan un booleano, que nos dice si la función que se evaluó era cierta o falsa. Existen seis condicionales que se pueden usar:
- \> ( Mayor que )
- < ( Menor que )
- \>= ( Mayor o igual que )
- <= ( Menor o igual que )
- != ( Diferente que )
- == ( Igual que )

Utilizando estos operadores, podemos evaluar ciertas condiciones. Por ejemplo, el siguiente código: 
```
Program conditionals;

Tabby() {
   Write(3 < 2, "\n");
   Write(3 > 2, "\n");
   Write(3 >= 2, "\n");
   Write(3 <= 2, "\n");
   Write(3 != 2, "\n");
   Write(3 == 2, "\n");
}
```

Imprimirá el siguiente resultado:
```
False
True
True
False
True
False
```

### Operaciones Lógicas
Las operaciones lógicas usualmente se utilizan con variables booleanas, y nos permiten comparar si dos o más condiciones se cumplen, o si al menos una de ellas cumple. Estas son AND, y OR. 
AND regresa True si ambas variables son True, False en otro caso
OR regresa True si al menos una variable es True, False en otro caso. 
Por ejemplo, el siguiente código: 
```
Program logic;

Tabby() {
   Write(True Or False, "\n");
   Write(True And False, "\n");
   Write(False Or False, "\n");
   Write(True And True, "\n");
}
```

Dará como resultado:
```
True
False
False
True
```
### Precedencia de Operaciones 
Todas estas operaciones pueden ser utilizadas en conjunto para evaluar expresiones. Usualmente, estas expresiones tienen cierto orden en el que se van evaluando, en las que algunas tienen prioridad sobre otras para ejecutarse antes. A continuación vemos la tabla de precedencia, desde lo más importante (que se resuelve antes) hasta lo último

| Operador | Ejemplo |
|---|---|
| ( ) | ( a + b ) * a |
| * /  | a * b / c |
| + - | a + b - c |
| >, <, >=, <=, !=, == | a > b, b != c |
| And | a And b |
| Or | a Or b |
| = | a = b |


Además, todos estos operadores son asociativos izquierdos, por lo que si dos operaciones en el mismo nivel de precedencia se encuentran, se resuelve el de la izquierda primero. 

### Operaciones
Con todas las operaciones mencionadas anteriormente, podemos crear expresiones complejas, como el siguiente programa:
```
Program ops;

Tabby() {
   Write(10 * 3 * (5 + 4 - 3) > 12 * (3 - 1) - 1 * 0, "\n");
   Write(True And 12 > 3 Or 2 < 3, "\n");
}
```
### Condicionales
En Tabby, existe el condicional If. Este condicional espera evaluar a un booleano. Si esto es cierto, ejecuta lo que está dentro de los corchetes. si no lo brinca. Por ejemplo:
```
Program if;

Tabby() {
   If(12 > 3) {
      Write("In If\n");
   }
   Write("End\n");
}
```
El programa evalúa la expresión, y como es cierta, imprime “In If”, y luego sigue imprimiendo “End”. Si cambiaramos la condicional a menor que, sería falso, y solo imprimiría “End”. 

También existe el If / Else. Es igual al If, pero tiene dos opciones, una para cierto y otra para falso, y solo ejecuta una, nunca cero o dos, siempre una.  Por ejemplo: 
```
Program ifElse;

Tabby() {
   If(12 < 3) {
      Write("In If\n");
   } Else {
      Write("In Else\n");
   }
   Write("End\n");
}
```

En este caso, sólo podrá imprimir o “In If”, o “In Else”, dependiendo de si la condición es cierta o falsa. 
### Ciclos
Muchas veces queremos hacer algo repetitivo, y ahí es donde los ciclos son útiles. En Tabby, existen dos tipos de ciclos. For, y While. 
While es parecido al If, evalúa una condición. Si es cierta entra, y si no brinca al final. La diferencia es que si fue cierto, recorre el código, pero en vez de seguir hacia abajo, regresa a re-evaluar la condición y repetir el proceso.  Por ejemplo: 
```
Program while;

Var Int i;

Tabby() {
   i = 1;
   While(i < 50) {
      Write(i, ", ");
      i = i * 2;
   }
}
```

Este código imprimirá: `1, 2, 4, 8, 16, 32, `

El For es muy parecido, pero integra una asignación. En este caso del While, tuvimos que modificar la variable i, pues si no se haría infinito el ciclo, y usualmente siempre hay una variable que cambia en cada iteración del ciclo. El For, además de tener una condicional, tiene una asignación que se hace después de cada ejecución del código dentro de los corchetes. El código equivalente al anterior con For sería: 
```
Program for;

Var Int i;

Tabby() {
   i = 1;
   For(i < 50; i = i * 2) {
      Write(i, ", ");
   }
}
```

### Funciones
Además de todo lo visto anteriormente, Tabby permite la declaración de funciones. Las funciones nos ayudan a poder representar procesos repetidos en un mismo lugar. Las funciones en Tabby deben de ser declaradas antes de la función Tabby. Estas funciones pueden tener parámetros, y variables locales. Además, las variables pueden solo hacer un proceso y no regresa ningún valor (a esto le llamamos Void), o regresar un valor de algún tipo (Int, Float o Bool). Estas funciones pueden ser llamadas dentro del código con el identificador y paréntesis, y dentro de estos los parámetros. 

A continuación, vemos un código que ejemplifica algunas funciones en Tabby:
```
Program fns;

Fn Void sayHi() {
   Write("Hi\n");
}

Fn Int sumOne(Int x) {
   Return x + 1;
}

Fn Void swap(Int x, Int y) 
Var Int aux;
{
   aux = x;
   x = y;
   y = aux;
}

Tabby() {
   sayHi();
   Write(sumOne(2), "\n");
   swap(1, 2);
}
```

La primera función es de tipo Void, es decir no regresa ningún valor. Si se usara en una expresión, se marcaría un error de compilación. Esta solo puede ser usada por sí sola. El ejemplo llama la función dentro de Tabby, y esta función imprime “Hi”. La siguiente función es de tipo Int, esto quiere decir que regresa un valor entero. En este caso recibe un parámetro, y al valor le suma uno. Lo podemos utilizar en un Write(), pues al regresar un Int, representa un valor que se puede imprimir. Además, vemos el estatuto Return. Esto hace que la función termine y le da el valor de la expresión al lugar que se llamó. Los Returns no se pueden usar en funciones Void.

Finalmente, la tercera función genera una variable local. Antes de los corchetes, se pueden definir variables que se utilizan en la función. Al ejecutar la función existen, pero al terminar la función estas desaparecen, como los parámetros. Si un parámetro tiene el mismo nombre que una variable global, dentro de la función se usa la variable local.

Se puede dar cualquier número  de parámetros, y también se puede usar la recursividad, en la que la función hace uso de sí misma. 
### Arreglos y Matrices
Además de variables singulares, a veces es útil tener grupos de variables con el mismo nombre. Para esto, Tabby nos permite declarar arreglos y matrices, de manera similar a las variables. Se les da un identificador, y un tamaño para cada dimensión. Se puede acceder a cada posición a través de constantes o expresiones. Se pueden declarar de manera global o local, pero no como parámetros, a excepción de las funciones estadísticas y gráficas. Se definen como las otras variables, pero con el prefijo Var Arr antes del tipo, y solo uno por línea (no es válido declarar varios separados por comas). Por ejemplo:
```
Program arrsMats;

Var Arr Int arr[10];
Var Arr Int mat[3][4];

Tabby() {
   arr[2] = 5;
   arr[2 * 3] = arr[2];
   mat[0][0] = 1;
   arr[3] = mat[0][1];
}
```

En este código se muestra como se usan los arreglos y matrices en Taby. Se indexan en cero. Si tenemos un arreglo de tamaño 10, las posiciones válidas son de 0 a 9. Un uso común por ejemplo, es leer N números con un ciclo y almacenarlos en un arreglo, si N no se conoce en compilación. 
### Funciones Estadísticas
Tabby está orientado a estadística. Por esto, Tabby provee varias funciones estadísticas que se pueden utilizar como otras funciones, pero tienen nombre predeterminados, y pueden recibir arreglos como parámetros. Las funciones son las siguientes: 


| Función | Tipo de Retorno |
|---|---|
|MinFlt|Float|
|MinInt|Int|
|MaxFlt|Float|
|MaxInt|Int|
|RangeFlt|Float|
|RangeInt|Int|
|MeanFlt|Float|
|MeanInt|Float|
|ModeInt|Int|
|MedianFlt|Float|
|MedianInt|Int|
|StdDevFlt|Float|
|StdDevInt|Float|
|VarianceFlt|Float|
|VarianceInt|Float|


Todas reciben los mismos parámetros. Primero, el nombre de un arreglo. Si la función tiene un sufijo “Flt”, de tipo flotante. Si el sufijo es “Int”, de tipo entero. Además, dos números que determinan la posición menor y mayor del arreglo en el que se quiere buscar. Por ejemplo:
```
Program statistics;

Var Arr Int arr[10];
Var Arr Float arrf[10];
Var Int i, j;

Tabby() {
   Write(MinInt(arr, 0, 3));
   Write(StdDevFlt(arrf, i, j));
}
```

### Funciones Gráficas 
Por último, Tabby también proporciona funciones para crear gráficas. La primera crea histogramas, y recibe como parámetros: un arreglo de datos (Int o Float), un número que representa el número de divisiones del histograma, el número de posiciones del arreglo a considerar (de 0 a n), y el nombre del archivo a generar.
Las otras dos para generar gráficas de líneas, y de dispersión. Estas reciben como parámetros: arreglo de datos en x, arreglo de datos en y, el número de posiciones del arreglo a considerar (de 0 a n), y el nombre del archivo a generar. Por ejemplo:
```
Program statistics;

Var Arr Int datax[10];
Var Arr Int datay[10];
Var Int n;

Tabby() {
   HistogramPlot(datax, 10, n, "hist");
   LinePlot(datax, datay, 5, "line");
   ScatterPlot(datax, datay, n / 2, "scatter");
}
```

