# RustCALC

Eksperimen kalkulator scientific desktop native menggunakan **100% Rust murni** dengan **egui / eframe** tanpa runtime web (Zero Tauri / WebView / JS). 

Dibuat untuk eksplorasi estetika **Neo-Brutalism** bergaya Hermès (Hermès Orange `#F37021`, border hitam tegas, shadow mekanik 3D) dan optimisasi performa ekstrem dengan konsumsi memori hanya **~0.3 – 2 MB RAM**.

---

### Fitur Ringkas
- **Math Engine (Pratt Parser)**: Aritmatika, preseden hierarkis, pangkat kanan (`2^3^2 = 512`), akar, perkalian implisit (`2pi`, `3(4+2)`).
- **Trigonometri & Logaritma**: Mode sudut dinamis `DEG` & `RAD`, hiperbolik, `ln`, `log10`, `log2`, faktorial (`5!`), persen (`%`).
- **Side Drawer**: Riwayat tape ledger (`[REC]`), lembar konstanta ($\pi, e, \phi, c, h, g, N_A$), dan konversi unit instan.
- **Dual Theme**: Mode terang `ALABASTER` dan mode gelap `ONYX`.
- **Keyboard & Clipboard**: Input numpad/keyboard fisik, auto-closing parentheses, dan copy/paste dinamis 1-klik.
- **Daftar Rumus Pengujian**: Lihat [FORMULA.md](FORMULA.md) untuk rumus siap copy-paste.

---

### Cara Menjalankan

#### Windows
```powershell
# Menjalankan aplikasi
cargo run --release

# Menjalankan unit test suite (10/10 passed)
cargo test
```
*Executable standalone Windows otomatis berada di `target/release/rustcalc.exe`.*

#### Linux (Arch Linux / Ubuntu / Fedora)
Aplikasi mendukung penuh sistem Linux berbasis X11 maupun Wayland.

**Arch Linux**:
```bash
# 1. Install dependensi grafis & compiler
sudo pacman -S --needed base-devel libx11 libxcursor libxrandr libxi libxkbcommon mesa pkgconf

# 2. Clone repository & jalankan
git clone https://github.com/Gustyx-Power/RustCALC.git
cd RustCALC
cargo run --release
```

**Ubuntu / Debian**:
```bash
sudo apt update && sudo apt install -y build-essential libx11-dev libxcursor-dev libxrandr-dev libxi-dev libxkbcommon-dev libgl1-mesa-dev pkg-config
cargo run --release
```

---
*Proyek eksperimental & coba-coba.*
