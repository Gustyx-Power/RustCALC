# RustCALC — Rumus Siap Salin (Copy-Paste)

Daftar lengkap rumus matematika scientific yang siap disalin (`Ctrl + C`) dan ditempel (`Ctrl + V` atau tombol `PASTE`) ke dalam **RustCALC** untuk pengujian manual maupun unit testing.

---

## 1. Aritmatika Dasar & Preseden Operasi

| Rumus | Mode | Hasil | Penjelasan |
| :--- | :--- | :--- | :--- |
| `(12.5 + 7.5) * 4 - 30 / 2` | DEG / RAD | `65` | Prioritas tanda kurung dan perkalian/pembagian |
| `2 + 3 * 4` | DEG / RAD | `14` | Perkalian diproses sebelum penjumlahan |
| `(2 + 3) * 4` | DEG / RAD | `20` | Tanda kurung mengubah urutan eksekusi |
| `1,000,000 + 250,500.25` | DEG / RAD | `1,250,500.25` | Mendukung angka dengan pemisah koma ribuan |
| `100 - 20 * 3 + 10 / 2` | DEG / RAD | `45` | Rantai operasi campuran |

---

## 2. Perkalian Implisit (Implicit Multiplication)

Tidak perlu mengetik tanda bintang/kali (`*`), engine mengenali perkalian implisit secara alami:

| Rumus | Mode | Hasil | Penjelasan |
| :--- | :--- | :--- | :--- |
| `2pi` | DEG / RAD | `6.283185307` | $2 \times \pi$ |
| `3(4 + 2)` | DEG / RAD | `18` | $3 \times (4 + 2)$ |
| `(5 + 3)(10 - 2)` | DEG / RAD | `64` | Perkalian antar kelompok kurung |
| `4sqrt(16)` | DEG / RAD | `16` | $4 \times \sqrt{16} = 4 \times 4$ |
| `2(3 + 1)pi` | DEG / RAD | `25.132741229` | Perkalian berantai implisit |

---

## 3. Pangkat & Akar

| Rumus | Mode | Hasil | Penjelasan |
| :--- | :--- | :--- | :--- |
| `2^3^2` | DEG / RAD | `512` | Asosiatif kanan: $2^{(3^2)} = 2^9 = 512$ |
| `(2^3)^2` | DEG / RAD | `64` | Kurung memaksakan asosiatif kiri: $8^2 = 64$ |
| `sqrt(144) + cbrt(27)` | DEG / RAD | `15` | Akar kuadrat ($\sqrt{144}=12$) + akar kubik ($\sqrt[3]{27}=3$) |
| `4^(1/2)` | DEG / RAD | `2` | Pangkat pecahan sebagai bentuk akar |
| `10^-3` | DEG / RAD | `0.001` | Pangkat negatif |

---

## 4. Trigonometri (Sudut DEG vs RAD)

> **Catatan**: Pastikan klik badge `[DEG]` / `[RAD]` di header kalkulator sesuai mode sudut yang diinginkan.

| Rumus | Mode yang Dibutuhkan | Hasil | Penjelasan |
| :--- | :--- | :--- | :--- |
| `sin(30) + cos(60)` | **DEG** | `1` | $\sin(30^\circ) = 0.5$ dan $\cos(60^\circ) = 0.5$ |
| `tan(45)` | **DEG** | `1` | $\tan(45^\circ) = 1$ |
| `asin(0.5)` | **DEG** | `30` | Invers sinus menghasilkan $30^\circ$ |
| `acos(0.5)` | **DEG** | `60` | Invers kosinus menghasilkan $60^\circ$ |
| `atan(1)` | **DEG** | `45` | Invers tangen menghasilkan $45^\circ$ |
| `sin(pi / 6)` | **RAD** | `0.5` | $\sin(\pi / 6)$ dalam radian |
| `cos(pi / 3)` | **RAD** | `0.5` | $\cos(\pi / 3)$ dalam radian |
| `cos(pi)` | **RAD** | `-1` | $\cos(\pi)$ dalam radian |

---

## 5. Fungsi Hiperbolik & Logaritma

| Rumus | Mode | Hasil | Penjelasan |
| :--- | :--- | :--- | :--- |
| `cosh(0) + sinh(0)` | DEG / RAD | `1` | Identitas hiperbolik: $\cosh(0)=1, \sinh(0)=0$ |
| `tanh(0)` | DEG / RAD | `0` | $\tanh(0) = 0$ |
| `ln(e^3)` | DEG / RAD | `3` | Logaritma natural basis $e$ |
| `log(10000)` | DEG / RAD | `4` | Logaritma basis 10 ($\log_{10}(10^4)$) |
| `log2(1024)` | DEG / RAD | `10` | Logaritma basis 2 ($\log_2(2^{10})$) |
| `e^(ln(42))` | DEG / RAD | `42` | Invers eksponensial dan logaritma |

---

## 6. Faktorial, Persentase & Kombinatorika

| Rumus | Mode | Hasil | Penjelasan |
| :--- | :--- | :--- | :--- |
| `5!` | DEG / RAD | `120` | Faktorial $5 \times 4 \times 3 \times 2 \times 1$ |
| `0!` | DEG / RAD | `1` | Definisi faktorial nol |
| `6! / (3! * 2!)` | DEG / RAD | `60` | Kombinasi / permutasi |
| `250 * 20%` | DEG / RAD | `50` | Evaluasi persentase ($250 \times 0.20$) |
| `50% + 50%` | DEG / RAD | `1` | Operasi persentase desimal ($0.5 + 0.5$) |

---

## 7. Konstanta Fisika & Matematika Universal

| Rumus | Mode | Hasil | Penjelasan |
| :--- | :--- | :--- | :--- |
| `phi^2 - phi` | DEG / RAD | `1` | Identitas rasio emas: $\phi^2 - \phi = 1$ |
| `c / 1000` | DEG / RAD | `299,792.458` | Kecepatan cahaya dalam ribuan km/s |
| `h * 10^34` | DEG / RAD | `6.62607015` | Konstanta Planck ($h$) |
| `g * 10` | DEG / RAD | `98.0665` | Percepatan gravitasi bumi ($g$) |
| `tau / pi` | DEG / RAD | `2` | Hubungan $\tau = 2\pi$ |

---

## 8. Fitur Auto-Closing Parentheses

Jika Anda malas mengetik kurung tutup `)` di ujung rumus panjang:

| Input | Tindakan | Hasil |
| :--- | :--- | :--- |
| `sqrt(sin(30) + (4 * 2` | Tekan `=` atau `Enter` | Otomatis melengkapi kurung penutup $\rightarrow$ `2.915475947` |
| `((2 + 3) * (4 + 1` | Tekan `=` atau `Enter` | Otomatis melengkapi 2 kurung $\rightarrow$ `25` |

