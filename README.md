# Rust para embebidos

Este proyecto debe servir como material de consulta para la creación de proyectos en Rust para sistemas embebidos.  Este primer proyecto tiene como objetivo mostrar las configuraciones básicas que se necesitan para un proyecto, la funcionalidad será solo la de hacer parpadear el LED integrado en el hardware de pruebas.

Posteriormente se harán más ejemplos con distintos periféricos y se tratará que sean lo más explicativos posibles.

- [Hardware de prueba](#hardware-de-prueba)
- [Instalación de Rust](#instalación-de-rust)
- [Preparación del proyecto](#preparación-del-proyecto)
	- [Crear un proyecto en Rust](#crear-un-proyecto-en-rust)
	- [Configuración global](#configuración-global)
	- [Linker script](#linker-script)
	- [Dependencias](#dependencias)
	- [Perfil para compilar el proyecto](#perfil-para-compilar-el-proyecto)
- [Aplicación](#aplicación)
	- [Código](#codigo)
	- [Compilar el proyecto](#compilar-el-proyecto)
	- [Debugging](#debugging)
	- [Openocd](#openocd)
	- [arm-none-gdb](#arm-none-gdb)
- [License](#license)

## Hardware de prueba

El hardware que se utiliza para realizar es una tarjeta de desarrollo NUCLEO-STM32L476, la cual cuneta con un microcontrolador STM32L476RG con las siguientes características:

* Núcleo Cortex-M4F (con unidad de punto flotante de precisión sencilla)
* Frecuencia de reloj de hasta 80 MHz
* 1 MB de Flash que empieza en la dirección 0x0800_0000 (Dividida en dos bancos de memoria)
* 128 KiB de SRAM que empieza en la dirección 0x2000_0000 (Dividida en dos bancos de memoria)

![Tarjeta de desarrollo](https://media.digikey.com/Photos/STMicro%20Photos/NUCLEO-L476RG.JPG)

## Instalación de Rust

La mejor manera para instalar Rust es la descrita en su página oficial [https://rustup.rs](https://rustup.rs). Para comprobar que tenemos instalado Rust ejecutamos el siguiente comando (la versión puede cambiar dependiendo la versión más reciente en ese momento).

~~~ j
$ rustc -V
rustc 1.31.1 (b6c32da9b 2018-12-18)
~~~

La instalación de Rust solo soporta compilación nativa, para poder tener soporte de "cross compilation" para procesadores con la arquitectura Cortex-M debemos agregar el target.

Cortex-M0, M0+, and M1 (ARMv6-M architecture):
~~~ j
$ rustup target add thumbv6m-none-eabi
~~~

Cortex-M3 (ARMv7-M architecture):
~~~ j
$ rustup target add thumbv7m-none-eabi
~~~

Cortex-M4 and M7 without hardware floating point (ARMv7E-M architecture):
~~~ j
$ rustup target add thumbv7em-none-eabi
~~~

Cortex-M4F and M7F with hardware floating point (ARMv7E-M architecture):
~~~ j
$ rustup target add thumbv7em-none-eabihf
~~~

## Preparación del proyecto

Para trabajar con Rust en sistemas embebidos es necesario preparar el proyectos con cierta configuración especifica para el hardware que vamos a usar. Lo primero que debemos hacer es agregar un archivo de configuración, este nos servirá para indicarle a Cargo (Cargo es el manejador de paquetes de Rust) información importante de nuestro proyecto, como puede ser la arquitectura del procesador.

### Crear un proyecto en Rust

Junto con la intalación de Rust, tendremos instalado Cargo que es el manejador de paquetes de Rust. Con ayuda de Cargo podemos generar protectos nuevos de rust. Para crear un proyecto debemos ejecutar el siguiente comando.

~~~ C
$ cargo new <project_name>
~~~

Este comando generá una caperta con todos los archivos necesarios para un proyecto de Rust.

~~~ C
new-project
├── Cargo.toml
└── src
    └── main.rs
~~~

### Configuración global

Primero debemos crear un carpeta dentro de la raíz nuestro proyecto llamada **.cargo**, esta carpeta funciona como "home" para Cargo, ahí se guardan distintos archivos que Cargo utilizará.

~~~ j
$ mkdir .cargo
~~~

Dentro de la carpeta **_.cargo_** debemos crear un archivo llamado **_config_**, Cargo automáticamente buscará este archivo lo agregará a las configuraciones globales del proyecto. Para nuestro proyecto el archivo debe contener lo siguiente.

~~~ C
$ tail .cargo/config
[build]
# Instruction set of Cortex-M4F (used in NUCLEO-STM32L476)
target = "thumbv7em-none-eabihf"

rustflags = [
  # use the Tlink.x scrip from the cortex-m-rt crate
  "-C", "link-arg=-Tlink.x",
]
~~~

Con las primeras líneas le decimos a Rust la arquitectura del target, con las demás instrucciones le decimos que usaremos el linker que viene por defecto (LLD). 

### Linker script

A grandes rasgos el **linker script** es el archivo que le dice el linker como es que tiene que unir todos los archivos generados después de compilar el proyecto para poder generar el archivo final que cargaremos a nuestro microcontrolador. De ahí que tengamos que generar dicho archivo para nuestro proyecto.

El contenido del linker script dependerá de las características de nuestro hardware, debemos generar un archivo llamado memory.x en la raíz de nuestro proyecto y el contenido debe ser el siguiente.

~~~ C
/* Linker script para STM32L476RG */
MEMORY
{
  RAM (xrw)      : ORIGIN = 0x20000000, LENGTH = 96K
  RAM2 (xrw)      : ORIGIN = 0x10000000, LENGTH = 32K
  FLASH (rx)      : ORIGIN = 0x8000000, LENGTH = 1024K
}
~~~

### Dependencias 

Cuando creamos un proyecto con Cargo se generan una sería de archivos por defecto, uno de ellos es un archivo llamado **Cargo.toml**, este archivo nos permite describir las dependencias que usaremos en nuestro proyecto, las dependencias no son más que los crates o librerías que utilizaremos en el proyecto. 

~~~ C
> tree
.
├── Cargo.lock
├── Cargo.toml
├── memory.x
├── README.md
├── src
    └── main.rs
~~~

Para cada proyecto las dependencias serán distintas, dependiendo del propósito del proyecto, para nuestro primer ejemplo debemos agregar las siguientes dependencias en el archivo Cargo.toml.
~~~ C
[dependencies]
# Nos da rutinas básicas de inicio para CPU ARM
cortex-m-rt = "0.6.7"
# Nos da acceso a registros propios de la arquitectura Cortex-M
cortex-m = "0.5.8"
# Contiene la rutina manejar "panic-halt" cuando tenemos errores
panic-halt = "0.2.0"
# Libreria que nos da funciones para tener acceso a los registros propios de
# familia de MCU que estemos usando
[dependencies.stm32l4]
version = "0.10.0"
features = ["stm32l4x6", "rt"]
~~~

### Perfil para compilar el proyecto

Otra configuración que debemos agregar al archivo Cargo.toml es el perfil con el cual vamos a compilar el proyecto, este perfil contiene, entre otras cosas, el nivel de optimización o si queremos agregar lo símbolos al para poder debuggear el proyecto.  

En el archivo Cargo.toml debemos agregar lo siguiente.

~~~ C
[profile.release]
# Optimización para el tamaño del archivo
opt-level = 's'
# Optimización de timepo para el linker.
lto = true
# Se habilita la funcion de debugging
debug = true
~~~


Estos serían todas las configuraciones para tener un proyecto básico, lo único que haría falta para poder compilar el proyeto sería agregar un código valido en el archivo **main. rs** que esta dentro de la carpeta **src/**, a continuación se muestra un código básico que podemos usar para probar que todas las configuraciones se realizaron de manera correcta.

~~~ Rust
#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to c    atch panics
extern crate cortex_m_rt;
extern crate cortex_m;
extern crate stm32l4;

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
       asm::nop(); // To not have main optimize to abort in release mode, remove wh    en you add code

    loop {
        // your code goes here
    }
}
~~~

Este código no tiene ninguna funcionalidad pero nos permitirá poder compilar el proyecto para comprobar que todo este funcionando de manera correcta. 

Para compilar el proyecto lo hacemos con la siguiente instrucción.

~~~ Rust
> cargo build --release
   Compiling typenum v1.12.0
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v1.0.12
   Compiling cortex-m v0.6.2
   Compiling stable_deref_trait v1.1.1
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.18
   Compiling vcell v0.1.2
   Compiling cortex-m-rt v0.6.12
   Compiling cortex-m-semihosting v0.3.5
   Compiling r0 v0.2.2
   Compiling stm32l4 v0.11.0
   Compiling panic-halt v0.2.0
   Compiling semver v0.9.0
   Compiling volatile-register v0.2.0
   Compiling rustc_version v0.2.3
   Compiling quote v1.0.4
   Compiling generic-array v0.13.2
   Compiling generic-array v0.12.3
   Compiling bare-metal v0.2.5
   Compiling as-slice v0.1.3
   Compiling aligned v0.3.2
   Compiling panic-semihosting v0.5.3
   Compiling cortex-m-rt-macros v0.1.8
   Compiling led-blink-stm32l4-PAC v0.1.0 (/home/alanmonu12/Documents/rust-embedded/led-blink-stm32l4-PAC)
warning: crate `led_blink_stm32l4_PAC` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `led_blink_stm32l4_pac`

    Finished release [optimized + debuginfo] target(s) in 1m 18s
~~~

## Aplicación 

Una vez teniendo el proyecto configurado para poder compilar para al arquitectura de nuestro microcontrolador solo nos queda escribir la aplicación que queramos realizar.

Para este primer acercamiento con Rust y los sistemas embebidos, la aplicación simplemente hace uso del LED integrado en la tarjeta de evaluación y lo hace parpadear con un periodo determinado. 

### Codigo 

El programa base es el siguiente.

~~~ Rust
#![no_std]
#![no_main]
// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to c    atch panics
extern crate cortex_m_rt;
extern crate cortex_m;
extern crate stm32l4;

use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;
use stm32l4::stm32l4x6;

#[entry]
fn main() -> ! {
    let peripherals = stm32l4x6::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    let rcc = &peripherals.RCC;
    hprintln!("Perifericos creados").unwrap();
    
    // Se debe habilitar el reloj para el GPIOA
    rcc.ahb2enr.write(|w| w.gpioaen().set_bit());
    
    // Se configura el periferico para funcionar como salida
    gpioa.moder.write(|w| w.moder5().bits(0b01));
    gpioa.otyper.write(|w| w.ot5().bit(false));
    gpioa.ospeedr.write(|w| w.ospeedr5().bits(0b00));
    gpioa.pupdr.write(|w| unsafe{w.pupdr5().bits(0b00)});
    hprintln!("Se configuro el pin 5 del puerto A").unwrap();
    
    loop {
       gpioa.bsrr.write(|w| w.bs5().set_bit());
       cortex_m::asm::delay(2000000);
       gpioa.bsrr.write(|w| w.br5().set_bit());
       cortex_m::asm::delay(2000000);
    }
}
~~~

### Compilar el proyecto

Para compilar el proyecto solo es necesario ejecutar el siguiente comando.
~~~ Rust
> cargo build --release
~~~

### Debugging
Para verificar que nuestro proyecto funciona tenemos la posibilidad de usar dos herramientas para el debugging:
* arm-eabi-gdb
* openodc (Open On-Chip Debugger)

#### Openocd
Openocd es un software que nos permite hacer [on-chip debugging](https://en.wikipedia.org/wiki/In-circuit_emulation#On-chip_debugging "In-circuit emulation"), [in-system programming](https://en.wikipedia.org/wiki/In-system_programming "In-system programming") y [boundary-scan](https://en.wikipedia.org/wiki/Boundary_scan "Boundary scan") para una amplia familia de arquitecturas. [[instalacion openocd]](http://testdiego.github.io/blog/2014/08/06/instalando-openocd/)

Para hacer más fácil el uso de openocd, vamos a generar un archivo de configuración que nos permita decirle al software que tipo de debugger estamos usando y cual target. En la raíz de proyecto generamos un archivo llamado **openocd.cfg** con el siguiente contenido.

~~~ C
# Esta configuracion cambia dependiendo la version de hardware
source [find interface/stlink-v2-1.cfg]
# Se debe selecionar el la familia del MCU que estemos usando
source [find target/stm32l4x.cfg]
~~~

La interfaces que usamos es el stlink v2.1 que viene incluido en las tarjetas de desarrollo NUCLEO-STM32.

![stlink v2.1 incluido en la tarjeta](https://jeelabs.org/wp-content/uploads/2015/11/DSC_5257.jpg)
Para comprobar que es posible conectarnos con el programador, podemos correr el siguiente comando, asegurándonos que la tarjeta de desarrollo está conectada a la PC.

~~~ C
> openocd
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 500 kHz
adapter_nsrst_delay: 100
none separate
Info : Unable to match requested speed 500 kHz, using 480 kHz
Info : Unable to match requested speed 500 kHz, using 480 kHz
Info : clock speed 480 kHz
Info : STLINK v2 JTAG v31 API v2 SWIM v21 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 3.258153
Info : stm32l4x.cpu: hardware has 6 breakpoints, 4 watchpoints
~~~

Si no tenemos ningún error eso quiere decir que ya estamos conectados con la el programador y podemos comenzar a enviarle archivos o comandos al MCU.

#### arm-none-gdb
GDB es un debugger que se distribuye junto con el proyecto GNU y nos permite ver el flujo de un programa en ejecución. [[instalacion arm-none-gdb]](https://stackoverflow.com/questions/53450745/message-unable-to-run-arm-none-eabi-gdb-cannot-find-libncurses-so-5)

Lo que vamos realizar es una conexión entre la sesión de Openocd y la sesión de GDB, lo que nos permitirá debuggear el código que estemos realizando.

De la misma manera que con Openocd, la manera más sencilla es crear un archivo de configuración para GDB, el cual tendrá los comandos básicos para tener una sesión de debugging y conectarnos con la sesión de Openocd. El archivo se debe crear en la raíz del proyecto con el nombre **openocd.gdb** y tener el siguiente contenido.

~~~ C
# Con este comando nos conectamos a la sesión de openocd
target remote :3333

set print asm-demangle on

# Se pone un break point en la funcion main
break main

# Se habilita el semihosting
monitor arm semihosting enable

# Se carga el programa al MCU
load

# Con este comando se continua la ejecución del programa 
c

# Se activa el cliente grafico de GDB
tui enable
~~~

La última configuración que debemos hacer para poder empezar a probar el programa es agregar al las siguientes lineas al archivo de configuración de cargo (.cargo/config).

~~~ C
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
#uncomment ONE of these three option to make `cargo run` start a GDB session
runner = "arm-eabi-gdb -x openocd.gdb"
 ~~~

Para iniciar una sesión de debugging solo es necesario ejecutar los siguiente comando.

En una terminal iniciamos la sesión de openocd.
~~~ C
> openocd
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 500 kHz
adapter_nsrst_delay: 100
none separate
Info : Unable to match requested speed 500 kHz, using 480 kHz
Info : Unable to match requested speed 500 kHz, using 480 kHz
Info : clock speed 480 kHz
Info : STLINK v2 JTAG v31 API v2 SWIM v21 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 3.258153
Info : stm32l4x.cpu: hardware has 6 breakpoints, 4 watchpoints
~~~


En otra terminal diferente ejecutamos lo siguiente.

~~~ C
> cargo run target/thumbv7em-none-eabihf/release/led-blink-stm32l4-PAC
~~~
Ese comando se conectara con la sesión de openocd, cargará el archivo *target/thumbv7em-none-eabihf/release/led-blink-stm32l4-PAC* al microcontrolador e iniciara un interfaz gráfica sencilla donde podremos seguir el flujo del programa.

![Sesion de GDB](https://i.ibb.co/TRKfbkb/Screenshot-from-2020-05-02-18-16-53.png)

Para continuar con la ejecución del programa escribimos en la sesión de GDB el comando continue.

~~~ GDB
(gdb) continue
~~~

Con esto el LED de la placa comenzará a parpadear.

## License
MIT
