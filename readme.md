# Tarea Hopcroft

## Resumen

Joseph Stuart Valverde Kong (C18100)

joseph.valverdekong@ucr.ac.cr 

Este repositorio se puede encontrar en: 

## Manual de uso

### Correr el programa

El programa se corre a través del siguiente comando desde la carpeta raiz.

```
cargo run
```

Con el cual se realiza la compilación y de inmediato se ejecuta el programa. 
Al realizar esto, se utiliza una configuración por defecto donde carga un automata desde el archivo nodes.txt y escribe los resultados en el archivo results.txt.

De querer utilizar otros archivos, se pueden pasar los nombres de estos a traves de parámetros de la siguiente manera.

```
cargo run [nombre_archivo_de_carga] [nombre_archivo_destino]
```

### Estructura de archivo

Los archivos de automatas, ya sean de origen o resultados, siguien todos la misma estructura.

- El abecedario
- Los estados
- Punto de origen
- Estados de acceptación
- Transiciones (todas las que sean necesarias)

### Otros datos a considerar

Debido a la manera en que se agrupan los estados, es posible que el estado inicial no tenga el mismo nombre, esto es normal. 
