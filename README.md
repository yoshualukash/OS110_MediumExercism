# OS110_MediumExercism
#### Yoshua Lukas - 1313617021 
#### Username on [Exercism](https://exercism.io) : yoshualukash 
Repositori ini dibuat untuk memenuhi tugas mata kuliah Operating System semester 110 Ilmu Komputer UNJ, tentang
Problem Solve Medium Level Rust yang ada pada [Rust Exercism](https://exercism.io/my/tracks/rust) .

5 Medium Problem yang saya kerjakan:
  1. Hamming
  2. Isogram
  3. Perfect Numbers
  4. Pythagorean Triplet
  5. Scrabble Score

***
# Isogram
Saya akan menjelaskan bagaimana saya menyelesaikan problem Isogram.

Code:
```rust
use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    let mut list_huruf = HashSet::new();
    if candidate.len() == 0{
        return true
    }
    else{
        let candidate_low = candidate.to_lowercase();
        let candidate_new = candidate_low.chars();
        for huruf in candidate_new{
            if huruf.is_alphabetic(){
                if list_huruf.contains(&huruf) {
                    return false
                }
                else{
                    list_huruf.insert(huruf);
                }
            }
        }
        return true
    }
}
```
## Penjelasan problemnya
Dalam problem isogram, kita diminta untuk membuat program yang dapat mengenali apakah suatu kata memiliki pengulangan huruf(isogram)  atau tidak.

Contoh kata yang termasuk isogram :
  + lumberjacks 
  + background
  + downstream
  + six-year-old

## Solusi saya
Tahapan saya dalam menyelesaikan problem tersebut:
1. Cek apakah inputan adalah String kosong atau tidak.
2. Ubah semua huruf dalam kata menjadi huruf kecil agar tidak terjadi perbedaan maksud. 
3. Input kata berupa String, agar dapat mengecek satu persatu huruf dalam kata maka input harus diubah menjadi tipe char.
4. Membuat hashset untuk memasukkan char setelah selesai membandingkan huruf.
5. Pastikan setiap char adalah sebuah huruf alphabet(bukan simbol ataupun angka).
6. Cek setiap char apakah sudah ada di dalam hashset list huruf. Jika belum, maka masukkan huruf tersebut kedalam hashset. Jika sudah ada maka otomatis kata tersebut bukan kata isogram.

Specific Code per tahapan:

1. Algoritmanya :
```rust
if candidate.len() == 0{
        return true
    }
```
untuk mengecek apakah inputan merupakan string kosong atau tidak. Dalam test code dijelaskan bahwa string kosong termasuk isogram, maka program tersebut me-return true.

2. `.to_lowercase()` untuk mengubah semua huruf menjadi huruf kecil.
3. `.chars()` untuk mengubah tipe data string menjadi char.
4. `use std::collections::HashSet;` dan 
  ```rust 
  let mut list_huruf = HashSet::new() 
  ```
  untuk membuat hashet yang nanti berisikan huruf.
  

5. `.is_alphabetic()` untuk mengecek apakah setiap huruf merupakan huruf alphabet.
6. Algoritmanya :
```rust
for huruf in candidate_new{
            if huruf.is_alphabetic(){
                if list_huruf.contains(&huruf) {
                    return false
                }
                else{
                    list_huruf.insert(huruf);
                }
            }
        }
        return true
```
Algoritma untuk menyeleksi huruf per huruf dalam kata tersebut.
Algoritma tersebut akan berhenti jika:
- Semua huruf dalam kata tersebut selesai di seleksi. -> me-return true (kata tersebut adalah kata isogram).
- Ada huruf yang sama setelah di cek dalam hashset list huruf. -> me-return false (kata tersebut bukan kata isogram).

`.contains(&huruf)` untuk mengecek suatu huruf apakah sudah ada di dalam hashset list huruf.

`.insert(huruf)` untuk memasukkan huruf kedalam hashset.

***
Demikian penjelasan saya mengenai medium level problem Isogram. Terima Kasih !

