# Tabby

Repositorio para el Lenguaje Tabby, un lenguaje enfocado en estadística implementando en Rust, desarrollado para la clase de Compiladores en el Semestre Ene-Jun 2022

<p align="center"><img src="./tabby.png" width="300"/></p>

## Avances

En esta seccción, se describen los avances conforme fueron entregados

### Avance 1: Léxico / Sintaxis

Para este avance, se creó el analizador léxico y sintáctico para el lenguaje Tabby. Para implementarlo, se utilizó la herramienta LALRPOP. La implementación de esta herramienta y su documentación se encuentran en los siguiente links:

> https://github.com/lalrpop/lalrpop 
> https://lalrpop.github.io/lalrpop/index.html

La gramática utilizada se puede encontrar el el documento `Propuesta Inicial Compilador`, en el folder `./docs/`
En este documento se muestran los diagramas de sintaxis de la gramática, para definir cada elemento de esta.
LALRPOP define cada objeto de la gramática como una estructura, la cuál puede tener varios caminos, como en los no terminales, o puede estar definido por una expresión regular como en el caso de los terminales.
Además, LALRPOP nos permite crear tests para cada objeto de la gramática, por lo que es fácil ir probando cada objeto de menor a mayor. Se prueba la lógica integral en su objeto, y al usarse en un objeto más grande, no es necesario probar esto otra vez completamente, si no ver si su uso es permitido o no. Esto simplifica mucho el proceso de pruebas y facilita encontrar bugs en la gramática

LALRPOP tiene un analizador léxico predeterminado que ignora los espacios en blanco, el cual es perfecto para Tabby. Esto solo nos deja con la implementación del analizador sintáctico.

Al momento de la entrega, el analizador sintáctico está completamente probado y completamente funcional para detectar si un programa está escrito correctamente en Tabby o no. Las pruebas se pueden encontrar en el folder `./tests/`. Las pruebas implementadas están en el archivo `tests.rs`. Además, existe un archivo llamado `customTest.tabby`, que puede ser modificado libremente para probar el analizador. Si se desea correr el analizador, se debe de utilizar el comando `cargo run`. El output nos dirá si el programa es correcto, o si se encontró un error de sintaxis en este. Este comando funciona al momento de entregar el avance 1, pero podría cambiar en avances futuros la manera de probarlo.

En futuras entregas, se agrega el análisis semántico y toda la lógica que falta en el compilador, pues este solo es el primer acercamiento a todo lo que se busca lograr

### Avance 2: Semántica Básica de Variables y Cubo Semántico

Para este avance, se pidió la implementación de dos componentes principales. La primera era la semántica básica de variables, lo cuál incluye la creación de una tabla de funciones, en las que almacenamos información de nombres y tipos de funciones, además de información de sus variables locales (estas se almacenan utilizando otra tabla completamente dentro de este objeto). De esta manera, podemos detectar errores semánticos como múltiple declaración de identificadores, tanto para variables globales o locales, como para nombres de funciones.
El otro componente del avance fue la creación de un "cubo semántico". Esta estructura nos ayudará a saber que tipo de operaciones son válidas entre diferentes tipos de datos, y también nos permite saber el tipo de resultado que se obtendrá de la operación.

Lo primero que se creó en este avance fue un AST (Abstract Syntax Tree). Con la herramienta utilizada para el analizador léxico y sintáctico no era posible integrar código a la par. Por esto, se tuvo que generar una estructura AST como resultado del Parser. Teniendo este AST, es posible ahora si hacer el análisis semántico, y se hace de una manera mucho más estructurada.
Esta parte generó el código en los archivos `ast.rs` y `ast_evaluator.rs`, los cuáles representan la estructura usada para el AST, y la evaluación de esta, en dónde podemos incluir puntos neurálgicos e información relevante para el análisis.
Además, se crearon casos de prueba en el archivo `tests.rs` (que cambió su directorio a `src/`), todos terminando con el sufijo `ast`. Se pueden probar con el comando `cargo test ast`, y evalúan que las estructuras almacenadas por el AST sean las esperadas, desde los elementos pequeños y separados, hasta los programas completos.

Después de implementar el AST, ahora sí fue posible generar los directorios de funciones y variables. Para esto se crearon los archivos `dir_func.rs` y `dir_var.rs`, los cuáles tienen estructuras y métodos utilizados para el almacenamiento de la información extraída del programa de forma estructurada. No solo se crearon estas estructuras, sino que se agregaron los puntos neurálgicos necesarios para la creación de estas estructuras y la adición de datos a estas, así como la detección de problemas de declaración múltiple en el mismo alcance. Además de la creación, se agregaron casos de prueba al módulo `test` (en el mismo archivo `tests.rs`), con el prefijo `dirs_`. Dependiendo del caso, se espera que funcione y se comprueba que la información sea correcta, o se espera que el código nos de errores si existen errores semánticos. Todos estos casos utilizan códigos escritos en tabby, los cuales están almacenados en `tests/`

Por último, se generó un archivo `semantic_cube.rs`, en el cuál se creo la estructura del cubo semántico, el cuál contiene toda la información necesaria para poder interpretar cualquier operación, dando información acerca de si la operación es válida o no, además del tipo de resultado que obtendremos de la operación realizada. Esta estructura fue creada, pero todavía no se utiliza en el código, pues hacen falta algunas partes para poder utilizarla de forma correcta.

Como comentario, en esta entrega se decidió remover el soporte para el tipo `Char`, pues no se utilizaba en nada, y su existencia no aportaba nada al lenguaje. Es por esto que simplemente se removió de la gramática

Cabe mencionar que además de estos avances de código, también se trabajó en la documentación, en la cuál se agregan las bitácoras, e información que se va generando y obteniendo al avanzar el código

### Avance 3. Generación de Código de Expresiones Aritméticas y Estatutos Secuenciales (Asignación, Lectura, Escritura)

Para la entrega de esta semana, se pidió que se agregara el código necesario para generar el código intermedio que se utilizará en una máquina virtual para el compilador.
Para esto, se creó un enum llamado `Quadruples`, el cuál se usará en un vector, para guardar los cuádruplos obtenidos del código. Este tiene varios tipos, ya sea para operaciones (`Op`), o algo más específico como un estatuto secuencial de lectura (`Read`) o de escritura (`Write`). Para ir llenando este vector, se utilizan varias pilas, que incorporan diferentes herramientas, como la validación del cubo semántico, para asegurar que las operaciones realizadas sean válidas y la generación de errores del tipo `type mismatch` en caso de ser necesario. También se utiliza una pila para guardar valores, ids y variables temporales que se utilizan en las expresiones aritméticas y en los cuádruplos para su evaluación. Por último se utiliza una pila para la validación de los operadores. Esta no es necesaria por el cómo está modelado la gramática y nuestro AST, pues la precedencia ya se resuelve con los niveles de la gramática y todos los operadores son asociativos izquierdos, pero igual se implementó por seguir las prácticas comunes, y por si es necesario implementar algún operador nuevo en el futuro, es más extensible. 
Para poder obtener los tipos de variables en expresiones o estatutos lineales, es necesario consultar las tablas de variables generadas en la entrega pasada. Para Tabby, se comprueban primero las variables locales y luego las globales. Si no está en ninguna de estas tablas, se imprime un error de `undeclared variable`, asegurándonos así de que las variables usadas siempre estén declaradas. También se hace lo mismo con las llamadas a funciones. Se busca en la tabla de funciones, y si no se encuentra se muestra un error de `undeclared function`. 
Además, como es costumbre, se agregaron casos de prueba para todas las funcionalidades nuevas agregadas. Se agregaron casos para comprobar la generación de cuádruplos para cada operando y expresión individualmente, además de algunas expresiones más complejas con paréntesis y diferentes precedencias, para verificar el correcto funcionamiento de la generación de cuádruplos.  También se generaron casos de prueba que se aseguran que el compilador detecte los errores de variables no declaradas y type mismatch como es debido. 
Como nota, todavía no funcionan los arreglos y matrices, pues eso es un tema más complejo y se agregará hasta más adelante. Tampoco se han implementado los estatutos no lineales, aunque son el objetivo de la implementación de la siguiente semana. 

### Avance 4. Cuádruplos en Estatutos No Lineales y Direcciónes de Memoria
Para la entrega semanal, se pidió que se agregara la generación de cuádruplos para estatutos no lineales. En Tabby, existen 4 estatutos que no siguen un flujo lineal. Estos son If, If/Else, While y For. Para implementar estos, se agregaron nuevas variantes del enum `Quadruples`, entre los cuales están `GoTo` o `GotoF`, que nos ayudan a brincar en un orden no lineal entre los cuádruplos. Para la implementación, se agregaron los puntos neurálgicos necesarios en el AST, generando así los cuádruplos a la par de la evaluación semántica, en el orden necesario. 
En la entrega pasada, los cuádruplos habían sido diseñados para solo contener los nombres de las variables utilizadas, pero esto en ejecución será más complicado de lo necesario y no será suficiente por falta de información, ya sea de tipo, scope, etc. Para esto, se asigna una memoria virtual a cada elemento de los cuádruplos, que ya tienen información codificada dentro. Dependiendo de la posición, podemos determinar scope y tipo. Esto simplificará el trabajo futuro. 
Para poder soportar esto, se cambió la definición de las variables de `Quadruples`, para recibir una tupla `(string, i32)`, representando el nombre y la dirección virtual asignada a esa variable. Para la asignación de esta memoria, se creó una estructura llamada `VirMemAllocator`, que se encarga de mantener la información y límites de la memoria, y de manejar el almacenamiento de las variables que van llegando de forma dinámica conforme se explora el ast y se checa la semántica. 
Todo el nuevo código fue probado extensamente con muchos casos de prueba nuevos, y la modificación de varios existentes para incluir la comprobación de estos nuevos elementos. 
Los siguientes pasos involucran generar los cuádruplos para las funciones, con las variaciones y nuevas variantes de cuádruplos que generen. Esto se debe encargar del código dentro de funciones, y de las variables locales y parámetros. 

