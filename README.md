# OS-CLX-BAREMETAL

Bare-metal kernel i avancuar, modular, i ndërtuar në Rust për platformën CLX/XLC/WWWMMM/NDB/Stigma.

## Qëllimi
- Kernel i vërtetë bare-metal, pa varësi nga OS ekzistues
- Primitiva moderne: trust, resonance, ndb, stigma
- Arkitekturë modulare, e dokumentuar
- Ndërtim production-grade

## Strukturë
- src/: Të gjithë modulat e kernelit
- .cargo/config.toml: Konfigurim për ndërtim bare-metal
- Cargo.toml: Konfigurim i varësive dhe profileve

## Ndërtimi

```sh
cargo build --release
```

## Migrimi
Ky repo përmban të gjithë kodin e migruar nga OS-CLX-MINIMAL dhe OS-CLX/kernel.

## Autorë
- UltraThinkingWeb / Clisonix AI

---
Për çdo pyetje ose kontribut, kontaktoni autorin ose hapni një issue në GitHub.
