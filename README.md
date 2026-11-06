# RustCALC

Eksperimen kalkulator scientific desktop native Windows menggunakan **100% Rust murni** dengan **egui / eframe** tanpa runtime web (Zero Tauri / WebView / JS). 

Dibuat untuk eksplorasi estetika **Neo-Brutalism** bergaya Hermès (Hermès Orange `#F37021`, border hitam tegas, shadow mekanik 3D) dan optimisasi performa ekstrem dengan konsumsi memori hanya **~0.3 – 2 MB RAM**.

---

### Fitur Ringkas
- **Math Engine (Pratt Parser)**: Aritmatika, preseden hierarkis, pangkat kanan (`2^3^2 = 512`), akar, perkalian implisit (`2pi`, `3(4+2)`).
- **Trigonometri & Logaritma**: Mode sudut dinamis `DEG` & `RAD`, hiperbolik, `ln`, `log10`, `log2`, faktorial (`5!`), persen (`%`).
- **Side Drawer**: Riwayat tape ledger (`[REC]`), lembar konstanta ($\pi, e, \phi, c, h, g, N_A$), dan konversi unit instan.
- **Dual Theme**: Mode terang `ALABASTER` dan mode gelap `ONYX`.
- **Keyboard & Clipboard**: Input numpad/keyboard fisik, auto-closing parentheses, dan copy hasil 1-klik.

---

### Cara Menjalankan

```bash
# Menjalankan aplikasi
cargo run --release

# Menjalankan unit test suite (9/9 passed)
cargo test
```

### Standalone Binary
Binary Windows executable standalone siap pakai berada di:
```
target/release/rustcalc.exe
```

---
*Proyek eksperimental & coba-coba.*
