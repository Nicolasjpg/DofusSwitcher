# DofusSwitcher

**DofusSwitcher** es una app para windows que permite manejar las ventanas para multicuenta en Dofus Unity (funciona con retro pero fue diseñada pensando solamente en unity)

Desarrollado en **Rust** 🦀 utilizando `egui` y la API nativa de Windows para garantizar un consumo de recursos mínimo (CPU/RAM).

## Características Principales

* **Atajos de teclado:** Alterna entre personajes usando atajos de teclado (Por defecto `<` y `F2`, reasignables).
* **Drag & Drop:** Ordena la lista de personajes arrastrando y soltando para definir el orden del ciclo.
* **Iconos Personalizados:** Asigna iconos específicos a cada personaje. El app **recuerda la configuración** automáticamente usando un json
* **Modo Mini (Overlay):**
    * Interfaz flotante.
    * Se puede usar en modo **Vertical** u **Horizontal**.
* **Portabilidad:** Ejecutable único con icono embebido.

## 📥 Instalación y Uso

1.  Descarga la última versión desde la sección de **Releases**.
2.  Descomprime el archivo `.zip` en una carpeta (Ej: `Mis Documentos/DofusSwitcher`).
3.  Asegúrate de que la carpeta `icons` esté junto al ejecutable.
4.  Ejecuta `DofusSwitcher.exe`.

### Estructura de Carpetas recomendada:
```text
📂 MiCarpeta/
 ├── 📄 DofusSwitcher.exe
 ├── 📄 dofus_config.json  (Se crea solo al guardar cambios)
 └── 📂 icons/             (Tus imágenes .png para los personajes)
      ├── ocra.png
      ├── yopuka.png
      └── ...
