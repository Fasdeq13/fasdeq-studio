<div align="center">

<img src="https://cdn.jsdelivr.net/npm/simple-icons@latest/icons/rust.svg" width="90" height="90" alt="Rust logo"/>

# Fasdeq Studio

**The best IDE for low-level development.**
**Лучшая IDE для низкоуровневой разработки.**

*by [Fasdeq13](https://github.com/Fasdeq13)*

[![Rust](https://img.shields.io/badge/Rust-99.8%25-dea584?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](./LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](#)
[![Release](https://img.shields.io/github/v/release/Fasdeq13/fasdeq-studio?style=for-the-badge&color=8A2BE2)](https://github.com/Fasdeq13/fasdeq-studio/releases)
[![Stars](https://img.shields.io/github/stars/Fasdeq13/fasdeq-studio?style=for-the-badge&color=yellow)](https://github.com/Fasdeq13/fasdeq-studio/stargazers)

[English](#english) • [Русский](#русский)

</div>

---

<a id="english"></a>

## English

### About

**Fasdeq Studio** is a native, lightweight IDE built in Rust for **low-level development in C, C++, Assembler, and Rust**. It focuses on the tools systems programmers actually need — a fast syntax-highlighted editor, a built-in disassembler, a binary hex inspector with live patching, and instant instruction reference — without the memory footprint of Electron-based editors.

<div align="center">
<table>
<tr>
<td align="center" width="110">
<img src="https://cdn.jsdelivr.net/npm/simple-icons@latest/icons/c.svg" width="40" height="40" alt="C"/><br/>
<sub><b>C</b></sub>
</td>
<td align="center" width="110">
<img src="https://cdn.jsdelivr.net/npm/simple-icons@latest/icons/cplusplus.svg" width="40" height="40" alt="C++"/><br/>
<sub><b>C++</b></sub>
</td>
<td align="center" width="110">
<img src="https://cdn.jsdelivr.net/npm/simple-icons@latest/icons/rust.svg" width="40" height="40" alt="Rust"/><br/>
<sub><b>Rust</b></sub>
</td>
<td align="center" width="110">
<img src="https://img.shields.io/badge/-Assembler-2b2b2b?style=flat-square" alt="Assembler" height="28"/><br/>
<sub><b>Assembler</b></sub>
</td>
<td align="center" width="110">
<img src="https://cdn.jsdelivr.net/npm/simple-icons@latest/icons/linux.svg" width="40" height="40" alt="Linux"/><br/>
<sub><b>Linux</b></sub>
</td>
</tr>
</table>
</div>

### Features

- 🖊️ **Code editor** — syntax highlighting for virtually any language (including hand-written support for Assembly and TOML), VSCode- or Vim-style keybindings, inline diagnostics with wavy underlines, and a JetBrains-style lightbulb quick-fix menu.
- 🔍 **Disassembler** — powered by Capstone, supports x86 (16/32/64-bit) and ARM/AArch64, with color-coded mnemonics by category (control flow, stack, arithmetic, logic, data movement, system).
- 🧬 **Binary inspector** — parses ELF and PE files, explains what every section does in plain language, and lets you view and patch raw bytes directly — save in place or as a patched copy.
- 🔢 **Number base converter** — live binary / octal / decimal / hexadecimal conversion with bit-width interpretation.
- 🧙 **First-run setup wizard** — detects your Linux distribution and installs Rust, GCC, G++, GDB, NASM, binutils, QEMU, CMake and Git straight from your distro's official repositories, or lets you point to existing binaries manually.
- 🎨 **Themes** — five built-in themes, plus support for loading your own theme from a CSS-like file or TOML.
- 🧩 **Extensions** — sandboxed WebAssembly plugins that can add new languages, themes, commands, and panels, or integrate outbound AI assistants such as Claude.
- 📦 **Project templates** — bare-metal OS starters in C & Assembler and C++ (16-bit real mode or 32-bit protected mode), a `no_std` Rust kernel starter, and minimal C/C++/Rust/Assembler projects.
- 🐧 **20+ Linux distributions supported** — Debian/Ubuntu family, Fedora/RHEL family, Arch family, openSUSE, Alpine, Void, Gentoo, NixOS, and more.

### Project structure

```
fasdeq-studio/
├── assets/              Icons, bundled syntax definitions, project templates, desktop entry
│   ├── icon/            Application icon (SVG source + rasterized PNGs)
│   ├── syntaxes/        Custom .sublime-syntax files (TOML, Assembly)
│   ├── templates/       Starter files used by the "New Project" wizard
│   └── themes/          Example custom CSS theme
├── docs/                Extension development guide
├── examples/extensions/ Sample WebAssembly extensions (Claude assistant, language support)
├── src/
│   ├── app.rs           Central application state
│   ├── main.rs          Entry point, window setup, icon loading
│   ├── disasm/          Disassembler engine, binary parser, hex view, instruction reference
│   ├── editor/          Text buffer, syntax highlighting, diagnostics, quick-fixes, Vim engine
│   ├── extensions/      WASM extension runtime, manifest parsing, extension manager
│   ├── project/         Project templates, recent projects, file tree model
│   ├── theme/           Theme definitions and application
│   ├── toolchain/       Distro detection, tool detection, installer
│   ├── ui/              All UI screens and panels (egui)
│   └── wizard/          First-run setup wizard state machine
├── Cargo.toml           Rust package manifest
├── install.sh           Builds and installs the binary, icon and desktop entry
└── LICENSE              MIT license
```

### Installation

Download the latest release, make it executable, and run it:

```bash
wget https://github.com/Fasdeq13/fasdeq-studio/releases/download/Fasdeq-Studio/fasdeq-studio
chmod +x fasdeq-studio
./fasdeq-studio
```

Or grab it directly from the [Releases page](https://github.com/Fasdeq13/fasdeq-studio/releases/tag/Fasdeq-Studio).

On first launch, Fasdeq Studio will detect your Linux distribution and offer to install any missing development tools automatically.

### Building from source

```bash
git clone https://github.com/Fasdeq13/fasdeq-studio.git
cd fasdeq-studio
cargo build --release
./target/release/fasdeq-studio
```

### License

Fasdeq Studio is licensed under the [MIT License](./LICENSE).

<div align="right">

[⬆ Back to top](#fasdeq-studio)

</div>

---

<a id="русский"></a>

## Русский

### О проекте

**Fasdeq Studio** — нативная, лёгкая IDE на Rust, созданная для **низкоуровневой разработки на C, C++, Assembler и Rust**. Проект сфокусирован на инструментах, которые реально нужны системным программистам: быстрый редактор с подсветкой синтаксиса, встроенный дизассемблер, hex-инспектор бинарников с живым патчингом и мгновенный справочник по инструкциям — без прожорливости редакторов на Electron.

<div align="center">
<table>
<tr>
<td align="center" width="110">
<img src="https://cdn.jsdelivr.net/npm/simple-icons@latest/icons/c.svg" width="40" height="40" alt="C"/><br/>
<sub><b>C</b></sub>
</td>
<td align="center" width="110">
<img src="https://cdn.jsdelivr.net/npm/simple-icons@latest/icons/cplusplus.svg" width="40" height="40" alt="C++"/><br/>
<sub><b>C++</b></sub>
</td>
<td align="center" width="110">
<img src="https://cdn.jsdelivr.net/npm/simple-icons@latest/icons/rust.svg" width="40" height="40" alt="Rust"/><br/>
<sub><b>Rust</b></sub>
</td>
<td align="center" width="110">
<img src="https://img.shields.io/badge/-Assembler-2b2b2b?style=flat-square" alt="Assembler" height="28"/><br/>
<sub><b>Assembler</b></sub>
</td>
<td align="center" width="110">
<img src="https://cdn.jsdelivr.net/npm/simple-icons@latest/icons/linux.svg" width="40" height="40" alt="Linux"/><br/>
<sub><b>Linux</b></sub>
</td>
</tr>
</table>
</div>

### Возможности

- 🖊️ **Редактор кода** — подсветка синтаксиса практически для любого языка (включая собственную поддержку ассемблера и TOML), стиль управления VSCode или Vim, встроенная диагностика ошибок волнистым подчёркиванием и меню быстрых исправлений в духе JetBrains (лампочка).
- 🔍 **Дизассемблер** — на базе Capstone, поддержка x86 (16/32/64-бит) и ARM/AArch64, мнемоники подсвечиваются по категориям (переходы, стек, арифметика, логика, работа с данными, системные).
- 🧬 **Инспектор бинарников** — разбирает ELF и PE файлы, объясняет назначение каждой секции простым языком, позволяет просматривать и патчить байты напрямую — сохранение на месте или как патченая копия.
- 🔢 **Конвертер систем счисления** — живая конвертация bin/oct/dec/hex с интерпретацией по разрядности.
- 🧙 **Мастер первого запуска** — определяет дистрибутив Linux и устанавливает Rust, GCC, G++, GDB, NASM, binutils, QEMU, CMake и Git из официальных репозиториев дистрибутива, либо позволяет указать пути к уже установленным бинарникам вручную.
- 🎨 **Темы** — пять встроенных тем, плюс возможность загрузить свою тему из CSS-подобного файла или TOML.
- 🧩 **Расширения** — песочница WebAssembly-плагинов, которые могут добавлять новые языки, темы, команды и панели, а также интегрировать внешних AI-ассистентов вроде Claude.
- 📦 **Шаблоны проектов** — стартовые ОС на C & Assembler и C++ (16-бит реальный режим или 32-бит защищённый режим), стартовое `no_std` ядро на Rust, минимальные проекты на C/C++/Rust/Assembler.
- 🐧 **Поддержка 20+ дистрибутивов Linux** — семейства Debian/Ubuntu, Fedora/RHEL, Arch, openSUSE, Alpine, Void, Gentoo, NixOS и другие.

### Структура проекта

```
fasdeq-studio/
├── assets/              Иконки, встроенные синтаксисы, шаблоны проектов, .desktop файл
│   ├── icon/            Иконка приложения (исходный SVG + растеризованные PNG)
│   ├── syntaxes/        Кастомные .sublime-syntax файлы (TOML, Assembly)
│   ├── templates/       Стартовые файлы для мастера "Создать проект"
│   └── themes/          Пример пользовательской CSS темы
├── docs/                Руководство по разработке расширений
├── examples/extensions/ Примеры WASM-расширений (Claude-ассистент, поддержка языка)
├── src/
│   ├── app.rs           Центральное состояние приложения
│   ├── main.rs          Точка входа, настройка окна, загрузка иконки
│   ├── disasm/          Движок дизассемблера, парсер бинарников, hex-вью, справочник инструкций
│   ├── editor/          Текстовый буфер, подсветка синтаксиса, диагностика, quick-fix, движок Vim
│   ├── extensions/      WASM-рантайм расширений, парсинг манифестов, менеджер расширений
│   ├── project/         Шаблоны проектов, недавние проекты, модель дерева файлов
│   ├── theme/           Определения тем и их применение
│   ├── toolchain/       Определение дистрибутива, детект инструментов, установщик
│   ├── ui/              Все экраны и панели интерфейса (egui)
│   └── wizard/          Машина состояний мастера первого запуска
├── Cargo.toml           Манифест Rust-пакета
├── install.sh           Собирает и устанавливает бинарник, иконку и .desktop файл
└── LICENSE              Лицензия MIT
```

### Установка

Скачай последний релиз, дай права на выполнение и запусти:

```bash
wget https://github.com/Fasdeq13/fasdeq-studio/releases/download/Fasdeq-Studio/fasdeq-studio
chmod +x fasdeq-studio
./fasdeq-studio
```

Либо скачай напрямую со страницы [Releases](https://github.com/Fasdeq13/fasdeq-studio/releases/tag/Fasdeq-Studio).

При первом запуске Fasdeq Studio определит твой дистрибутив Linux и предложит автоматически установить недостающие инструменты разработки.

### Сборка из исходников

```bash
git clone https://github.com/Fasdeq13/fasdeq-studio.git
cd fasdeq-studio
cargo build --release
./target/release/fasdeq-studio
```

### Лицензия

Fasdeq Studio распространяется под лицензией [MIT](./LICENSE).

<div align="right">

[⬆ Наверх](#fasdeq-studio)

</div>
