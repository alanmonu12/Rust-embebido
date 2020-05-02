# Rust embedded

Este proyecto debe servir como material de consulta para la creación de proyectos en Rust para sistemas embebidos. 

## Hardware de prueba

El hardware que se utliza para realizar es una tarjeta de desarrollo NUCLEO-STM32L476, la cual cuneta con un microcontrolador STM32L476RG con las siguientes caracteriticas:

* Nucleo Cortex-M4F (con unidad de punto flotante de precisión sencilla)
* Frecuancia de reloj de hasta 80 MHz
* 1 MB de Flash que empieza en la dirección 0x0800_0000 (Dividida en dos bancos de memoria)
* 128 KiB de SRAM que empieza en la dirección 0x2000_0000 (Dividida en dos bancos de memoria)

## Instalación de Rust

La mejor manera para instalar Rust es la descrita en su página oficial [https://rustup.rs](https://rustup.rs). Para comprobar que tenemos instalado Rust ejecutamos el siguiente comando (la version puede cambiar dependiendo la version más reciente en ese momento).

~~~
$ rustc -V
rustc 1.31.1 (b6c32da9b 2018-12-18)
~~~

La instalación de Rust solo soporta compilación nativa, para poder tener soporte de "cross compilation" para procesadores con la arquitectura Cortex-M debemos agregar el target.

**Cortex-M0, M0+, and M1 (ARMv6-M architecture):**
~~~
$ rustup target add thumbv6m-none-eabi
~~~

**Cortex-M3 (ARMv7-M architecture):**
~~~
$ rustup target add thumbv7m-none-eabi
~~~

**Cortex-M4 and M7 without hardware floating point (ARMv7E-M architecture):**
~~~
$ rustup target add thumbv7em-none-eabi
~~~

**Cortex-M4F and M7F with hardware floating point (ARMv7E-M architecture):**
~~~
$ rustup target add thumbv7em-none-eabihf
~~~


