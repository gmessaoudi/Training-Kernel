# Kernel Minimal Rust RISC-V

Un système d'exploitation expérimental ("bare-metal") écrit en Rust pur (`#![no_std]`, `#![no_main]`) ciblant l'architecture RISC-V.

Ce projet a été créé dans un but d'apprentissage pour explorer les interactions bas niveau avec le matériel, la gestion manuelle de la mémoire et la configuration matérielle sans l'aide d'une bibliothèque standard.

## Fonctionnalités implémentées

* **Bootloader minimaliste :** Séquence d'amorçage en assembleur (`entry.S`) qui configure la pile, nettoie la section BSS et saute vers le code Rust.
* **Pilote UART :** Implémentation complète d'un driver série avec lecture/écriture (Memory-Mapped I/O) et support des macros de formatage Rust (`print!`, `println!`).
* **Gestionnaire d'interruptions matérielles :** Configuration des registres CSR, gestion du mode Machine, interception des *Traps* et des exceptions (défauts de page, instructions illégales, etc.).
* **Timer matériel :** Utilisation de l'horloge système (MTIME / MTIMECMP) pour générer des interruptions périodiques.
* **Allocateur de mémoire dynamique (Heap) :** Implémentation du trait `GlobalAlloc` avec une zone mémoire de 1 Mo, utilisant un algorithme personnalisé de liste chaînée (Linked-list allocator) pour les allocations et désallocations.
* **Shell interactif :** Interpréteur de commandes basique permettant à l'utilisateur d'interagir avec le système via le port série (`help`, `echo`, `uptime`, etc.).

## Prérequis

Pour compiler et exécuter ce projet, vous aurez besoin de la toolchain Rust (version *nightly*) et de l'émulateur QEMU pour RISC-V.

```bash
# Installer la cible Rust pour RISC-V
rustup target add riscv32imac-unknown-none-elf

# Installer QEMU (Exemple sur Ubuntu/Debian)
sudo apt-get install qemu-system
```

## Compilation et Exécution

1. **Compiler le noyau :**
```bash
cargo build --release
```

2. **Lancer avec QEMU (Machine Virt) :**
```bash
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios none \
    -kernel target/riscv32imac-unknown-none-elf/release/kernel
```

3. **Compiler et lancer :**
```bash
cargo run --release
```

*Note : Pour quitter l'émulateur QEMU, appuyez sur `Ctrl+A` puis sur `X`.*

## Structure du projet

* `linker.ld` : Script de l'éditeur de liens définissant la carte mémoire (RAM à `0x80000000`).
* `src/arch/` : Code spécifique à l'architecture (Assembleur d'entrée, gestion des *Traps*).
* `src/hal/` : *Hardware Abstraction Layer* (abstractions MMIO).
* `src/drivers/` : Pilotes matériels (UART).
* `src/memory/` : Gestionnaires de la heap (Bump allocator & Linked-list allocator).
* `src/time/` : Gestionnaire de l'horloge système RISC-V.
* `src/utils/` : Parseur de commandes et logique du Shell.
