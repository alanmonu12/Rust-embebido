# Rust embedded

Este proyecto debe servir como material de consulta para la creación de proyectos en Rust para sistemas embebidos. 

## Hardware de prueba

El hardware que se utliza para realizar es una tarjeta de desarrollo NUCLEO-STM32L476, la cual cuneta con un microcontrolador STM32L476RG con las siguientes caracteriticas:

* Nucleo Cortex-M4F (con unidad de punto flotante de precisión sencilla)
* Frecuancia de reloj de hasta 80 MHz
* 1 MB de Flash que empieza en la dirección 0x0800_0000 (Dividida en dos bancos de memoria)
* 128 KiB de SRAM que empieza en la dirección 0x2000_0000 (Dividida en dos bancos de memoria)

## Instalación de Rust

La mejor manera para instalar Rust es la descrita en su página oficial [https://rustup.rs](https://rustup.rs). Para comprobar que tenemos instalado Rust ejecutamos el siguiente comando.

~~~
$ rustc -V
rustc 1.31.1 (b6c32da9b 2018-12-18)
~~~

