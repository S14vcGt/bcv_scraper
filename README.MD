# 🔄 Conversor de Tasa de Cambio del Bolívar (CLI) 💻

Este proyecto es una interfaz de línea de comandos (CLI) 🖥️ escrita en **Rust** 🦀 que utiliza técnicas de **web scraping** 🕸️ para obtener la tasa de cambio oficial del **Bolívar venezolano (VES)** a diferentes monedas 💵, según lo publicado por el **Banco Central de Venezuela (BCV)** 🏦.

---

## 📋 Tabla de Contenidos

1. [Descripción 📝](#descripción)
2. [Instalación 🚀](#instalación)
3. [Uso ⚙️](#uso)
4. [Ejemplo de Salida 📊](#ejemplo-de-salida)

---

## Descripción 📝

El programa consulta la página web oficial del **Banco Central de Venezuela (BCV)** para obtener la información más reciente sobre las tasas de cambio del Bolívar a otras monedas. Utiliza **web scraping** 🕷️ para extraer los datos relevantes y mostrarlos en un formato legible directamente desde la terminal 📋.

🔗 URL del sitio web del BCV: [https://www.bcv.org.ve/](https://www.bcv.org.ve/)

🔗 URL del paquete en crates.io: [https://crates.io/crates/bcv](https://crates.io/crates/bcv)
---

## Instalación 🚀

Puedes descargar el paquete de Cargo con el siguiente comando:

```bash
cargo install bcv
```
o si quieres compilarlo localmente, puedes seguir los pasos siguientes:

1. Clona este repositorio en tu máquina local 🖥️:

   ```bash
   git clone https://github.com/tu-usuario/conversor-bcv-cli.git
   cd conversor-bcv-cli
   ```

2. Compila el proyecto con Cargo 🏗️:

   ```bash
   cargo build --release
   ```

3. El archivo ejecutable estará disponible en la carpeta `target/release`. Puedes moverlo a un directorio incluido en tu variable de entorno `PATH` para facilitar su uso 📂:

   ```bash
   mv target/release/conversor-bcv /usr/local/bin/
   ```

---

## Uso ⚙️

Una vez instalado, puedes usar el programa ejecutando el siguiente comando 🔧:

```bash
bcv
```

El programa mostrará automáticamente las tasas de cambio disponibles para el Bolívar venezolano según el BCV 📈.


## Ejemplo de Salida 📊

Al ejecutar el programa, verás una salida similar a esta 🖥️:

```
📊 Tasas de Cambio del Bolívar (VES) - Banco Central de Venezuela 🏦
📅 Fecha: 2023-10-15

Moneda       | Tasa de Cambio
-------------|---------------
💵 Dólar (USD)  | 35.00
💶 Euro (EUR)   | 38.50
💴 Yuan (CNY)   | 5.10
...
```