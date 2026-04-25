# Native Notes Pad 📝

[English](README.md) | [Español]

[![License: MIT](https://img.shields.io/badge/Licencia-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Privacidad: 100% Local](https://img.shields.io/badge/Privacidad-100%25_Local-brightgreen?style=flat&logo=shield)
![SO: Windows](https://img.shields.io/badge/SO-Windows_10%2F11-blue?logo=windows)

Un bloc de notas extremadamente rápido, ligero y nativo para Windows escrito en **Rust**. 
Diseñado para evitar la telemetría y el bloatware, siendo una alternativa moderna, eficiente y minimalista al Bloc de notas clásico de Windows.

![image](assets/ram_memory.png)

## ✨ Características

- 🚀 **Super Ligero**: Escrito en Rust utilizando las APIs nativas de Windows (Win32).
- 🔒 **Privacidad absoluta y seguridad**: Sin telemetría ni bloatware, no se recoge ni un solo dato ni estadísticas. No se conecta a Internet.
- ⚡ **Rendimiento Extremo**: El mínimo consumo de memoria RAM posible, típicamente entre **1.8 MB y 2 MB**.
- 🚫 **Sin Bloatware**: Sin actualizaciones en segundo plano ni funciones que consuman recursos innecesariamente.
- 🌍 **Multilingüe**: Soporte para Español e Inglés (cambio en tiempo real).
- 🔍 **Zoom Integrado**: Ajusta el tamaño de la fuente fácilmente con `Ctrl` + `+` / `Ctrl` + `-` o usando `Ctrl` + `Rueda del Ratón`.
- 🔄 **Ajuste de Línea**: Activa o desactiva el ajuste de texto con un solo clic.
- 🎨 **Personalización de Fuente**: Elige la tipografía que prefieras usar.
- 💾 **Soporte UTF-16 y ASCII**: Manejo nativo de múltiples formatos de codificación para evitar problemas con caracteres especiales.
- 🖥️ **Para Windows 11 y Windows 10**: Totalmente optimizado para las últimas versiones de Windows.

## 📥 Descarga

Puedes descargar la última versión ya compilada (el ejecutable `.exe` listo para usar) desde las siguientes plataformas:

👉 **[Descargar en Itch.io - Paga lo que quieras] (https://mariolobato.itch.io/native-notes-pad)**

## 🛠️ Compilar desde el código fuente

Si prefieres compilar el proyecto tú mismo, asegúrate de tener [Rust y Cargo](https://www.rust-lang.org/tools/install) instalados.

1. Clona este repositorio:
   ```bash
   git clone https://github.com/mariolobato/native_notes_pad.git
   cd native_notes_pad
   ```

2. Compila el proyecto en modo **Release** para obtener un ejecutable optimizado:
   ```bash
   cargo build --release
   ```

3. El ejecutable se encontrará en:
   ```
   target/release/native_notes_pad.exe
   ```

## ❤️ Apoyar el proyecto

Este proyecto es de código abierto. Si te resulta útil en tu día a día, considera invitarme a un café para apoyar el desarrollo continuo:

- [Apoyar en Ko-fi](https://ko-fi.com/mariolobato)
- [Donar en Gumroad] (TODO: Añade enlace)

¡Gracias por usar Native Notes Pad!
