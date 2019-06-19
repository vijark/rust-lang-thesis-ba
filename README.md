# Bachelorarbeit:<br/>Programmieren in Rust und Vergleich mit C/C++

## Inhalt
1. Einleitung
    - Was ist Rust?
2. Rust toolchain
    - rustup
    - rustc
        - Grundlegende Verwendung
        - Lints
    - Cargo
        - Projektverwaltung
        - Veröffentlichung bei crates.io
        - Externe Tools
3. Programmierung mit Rust und Unterschiede zu C/C++
    - Grundlagen<br/>
    \[**Rust** [number_types](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/number_types/src/main.rs),
    [functions](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/functions/src/main.rs),
    [control_flow](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/control_flow/src/main.rs)\]<br/>
    \[**C/C++** [number_types](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/C_C%2B%2B/number_types/main.c)\]
        - Variablen und Mu­ta­bi­li­tät
        - Datentypen
        - Funktionen
        - Kontrollstrukturen
    - Ownership<br/>
    \[**Rust** [strings](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/strings/src/main.rs),
    [references](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/references/src/main.rs),
    [simple_linked_list](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/simple_linked_list/src/lib.rs),
    [threads](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/threads/src/main.rs)\]<br/>
    \[**C/C++** [threads/without_mutex](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/C_C%2B%2B/threads/without_mutex/main.c),
    [threads/with_mutex](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/C_C%2B%2B/threads/with_mutex/main.c),
    [threads/C++](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/C_C%2B%2B/threads/C%2B%2B/main.cpp)\]
        - Funktionweise von Ownership
        - Referenzen und Borrowing
        - Slice Typ
    - Modulsystem<br/>
    \[**Rust** [modules](https://github.com/vijark/rust-lang-thesis-ba/tree/master/code/Rust/modules/src)\]<br/>
    \[**C/C++** [header_files](https://github.com/vijark/rust-lang-thesis-ba/tree/master/code/C_C%2B%2B/header_files)\]
    - Objektorientierung<br/>
    \[**Rust** [rectangle](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/rectangle/src/main.rs),
    [employees](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/employees/src/main.rs)\]<br/>
    \[**C/C++** [rectangle](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/C_C%2B%2B/rectangle/main.cpp),
    [employees](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/C_C%2B%2B/employees/main.cpp)\]
        - Kapselung
        - Vererbung
        - Traits
    - Generische Programmierung<br/>
    \[**Rust** [generics](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/generics/src/main.rs)\]<br/>
    \[**C/C++** [generics](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/C_C%2B%2B/generics/main.cpp)\]
    - Unit-Tests<br/>
    \[**Rust** [testing](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/testing/src/main.rs)\]
    - Error Handling<br/>
    \[**Rust** [errors](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/errors/src/main.rs)\]<br/>
    \[**C/C++** [errors_in_C](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/C_C%2B%2B/errors_in_C/main.c),
    [errors_in_C++](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/C_C%2B%2B/errors_in_C%2B%2B/main.cpp)\]
        - Fehler mit panic!
        - Fehler mit Result
        - ?-Operator
        - Error Handling in C und C++
    - Dokumentieren mit rustdoc<br/>
    \[**Rust** [documenting](https://github.com/vijark/rust-lang-thesis-ba/blob/master/code/Rust/documenting/src/lib.rs)\]
        - Grundlegende Verwendung
        - Dokumentationstests
    - WebAssembly<br/>
    \[**Rust** [wasm](https://github.com/vijark/rust-lang-thesis-ba/tree/master/code/Rust/wasm)\]
        - Beispielprojekt in Rust anlegen
4. Performance
    - zero-cost in Rust
    - Benchmark-Tests
        - Die Benchmarkprogramme
        - Ausführungszeit
        - Speicherverbrauch
