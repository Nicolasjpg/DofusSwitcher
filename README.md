# DofusSwitcher

**DofusSwitcher** es una app para windows que permite manejar las ventanas para multicuenta en Dofus Unity (funciona con retro pero fue diseñada pensando solamente en unity), detecta automaticamente las ventanas de Dofus abiertas

Desarrollado en **Rust** utilizando `egui` y la API nativa de Windows para garantizar un consumo de recursos mínimo (CPU/RAM).

## Características Principales

* **Atajos de teclado:** Alterna entre personajes usando atajos de teclado (Por defecto `<` y `F2`, reasignables).
* **Drag & Drop:** Ordena la lista de personajes arrastrando y soltando para definir el orden del ciclo.
* **Iconos Personalizados:** Asigna iconos específicos a cada personaje. El app **recuerda la configuración** automáticamente usando un json
* **Modo Mini (Overlay):**
    * Interfaz flotante.
    * Se puede usar en modo **Vertical** u **Horizontal**.
* **Portabilidad:** Ejecutable único con icono embebido.

## Instalación y Uso

### Requisitos Previos
* Sistema Operativo: **Windows 10 o 11**
### Pasos
1.  Ve a la sección de **[Releases](../../releases)** de este repositorio.
2.  Descarga el archivo `.zip` de la última versión (ej: `DofusSwitcher_v1.0.zip`).
3.  Descomprime el archivo
4.  **IMPORTANTE:** Asegurarse que la carpeta `icons` esté junto al ejecutable.

### Estructura Correcta de la Carpeta
Para que los iconos funcionen, lacarpeta debe verse así:

```text
📂 DofusSwitcher/
 │
 ├── ⚙️ DofusSwitcher.exe      <-- El programa principal
 ├── 📄 dofus_config.json      <-- (Se crea automáticamente al guardar)
 │
 └── 📂 icons/                 <-- Carpeta OBLIGATORIA con tus imágenes
      ├── ocra.png
      ├── aniripsa.png
      ├── panda.png
      └── default.png
```
**Iconos Personalizables:** La carpeta `icons` está diseñada para poner cualquier imagen, idealmente en formato **.png** se puede personalizar al gusto del usuario.

## Créditos y Autor

> Este proyecto fue desarrollado por diversión con el objetivo de crear una alternativa **ligera** y específica a la función *Organizer* de [DofusGuide](https://dofusguide.fr/accueil). Aunque está ajustado a mis necesidades personales como jugador, siéntanse libres de clonar el repositorio y adaptar el código a sus propias necesidades.

* **Desarrollador:** **Exil** (Juego en los servidores *Rafal* y *Talkasha*).
* **Recursos Gráficos:** Los iconos utilizados en este proyecto fueron tomados de [E-bou - Galerie](https://api.e-bou.fr/img-browser), creada por [@Faareoh](https://x.com/Faareoh).

## Licencia y Aviso Legal

![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

**DofusSwitcher** es una herramienta segura:
* No inyecta código.
* No automatiza acciones (No es un bot).
* Solo gestiona ventanas de Windows.

> **Aviso:** El uso de software de terceros es responsabilidad exclusiva del usuario.
