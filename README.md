# Tabby

Repositorio para el Lenguaje Tabby, un lenguaje enfocado en estadística implementando en Rust, desarrollado para la clase de Compiladores en el Semestre Ene-Jun 2022

## Avances

En esta seccción, se describen los avances conforme fueron entregados

### Avance 1: Léxico / Sintaxis

Para este avance, se creó el analizador léxico y sintáctico para el lenguaje Tabby. Para implementarlo, se utilizó la herramienta LALRPOP. La implementación de esta herramienta y su documentación se encuentran en los siguiente links:

> https://github.com/lalrpop/lalrpop > https://lalrpop.github.io/lalrpop/index.html

La gramática utilizada se puede encontrar el el documento `Propuesta Inicial Compilador`, en el folder `./docs/`
En este documento se muestran los diagramas de sintaxis de la gramática, para definir cada elemento de esta.
LALRPOP define cada objeto de la gramática como una estructura, la cuál puede tener varios caminos, como en los no terminales, o puede estar definido por una expresión regular como en el caso de los terminales.
Además, LALRPOP nos permite crear tests para cada objeto de la gramática, por lo que es fácil ir probando cada objeto de menor a mayor. Se prueba la lógica integral en su objeto, y al usarse en un objeto más grande, no es necesario probar esto otra vez completamente, si no ver si su uso es permitido o no. Esto simplifica mucho el proceso de pruebas y facilita encontrar bugs en la gramática

LALRPOP tiene un analizador léxico predeterminado que ignora los espacios en blanco, el cual es perfecto para Tabby. Esto solo nos deja con la implementación del analizador sintáctico.

Al momento de la entrega, el analizador sintáctico está completamente probado y completamente funcional para detectar si un programa está escrito correctamente en Tabby o no. Las pruebas se pueden encontrar en el folder `./tests/`. Las pruebas implementadas están en el archivo `tests.rs`. Además, existe un archivo llamado `customTest.tabby`, que puede ser modificado libremente para probar el analizador. Si se desea correr el analizador, se debe de utilizar el comando `cargo run`. El output nos dirá si el programa es correcto, o si se encontró un error de sintaxis en este. Este comando funciona al momento de entregar el avance 1, pero podría cambiar en avances futuros la manera de probarlo.

En futuras entregas, se agrega el análisis semántico y toda la lógica que falta en el compilador, pues este solo es el primer acercamiento a todo lo que se busca lograr
