# Rust embedded

Este proyecto debe servir como material de consulta para la creación de proyectos en Rust para sistemas embebidos. 

## Hardware de prueba

El hardware que se utliza para realizar es una tarjeta de desarrollo NUCLEO-STM32L476, la cual cuneta con un microcontrolador STM32L476RG con las siguientes caracteriticas:

* Nucleo Cortex-M4F (con unidad de punto flotante de precisión sencilla)
* Frecuancia de reloj de hasta 80 MHz
* 1 MB de Flash que empieza en la dirección 0x0800_0000 (Diidida en dos bancos de memoria)
* 128 KiB de SRAM que empieza en la dirección 0x2000_0000 (Dividad en dos bancos de memoria)

