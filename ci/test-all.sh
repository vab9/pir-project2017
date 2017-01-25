#!/bin/bash

# This file needs to be run from the git root directory!

# Exit script on the first error
set -o errexit -o nounset

export RUSTFLAGS="--deny warnings"

# Jedes `sheetN/taskN` Verzeichnis enthält entweder ein Cargo-Projekt (also eine
# Cargo.toml) oder eine einzige `.rs`-Datei für Nicht-Cargo-Projekte die direkt
# mit rustc kompiliert werden.
#
# Um den Travis-Cache effizient zu nutzen (es wird `/target` gecached) und
# generell die CI-Zeiten zu verringern, wechseln wir nie das Verzeichnis und
# kompilieren immer nur die Aufgaben des aktuellsten Aufgabenblatts.

current_sheet=$()
#$(ls -d sheet* | tail -n1)

#if [ -z "$current_sheet" ]; then
#    echo ""
#    echo "=== Keine Aufgaben gefunden. Beende."
#    exit 0
#fi

echo ""
echo "=== Aktuelles Aufgabenblatt ist in Ordner $current_sheet"

manifest="rustle-my-net/Cargo.toml"
if [ -e "$manifest" ]; then
    echo "=== Cargo-Manifest gefunden in '$manifest' -> Cargo-Modus"
    cargo test --manifest-path "$manifest"
elif [ $(ls *.rs | wc -l) -ne 0 ]; then
    echo "=== Sourcedatei(en) gefunden -> rustc-Modus"
    for srcfile in *.rs; do
        echo "=== Kompiliere und teste '$srcfile'..."
        rustc "$srcfile"
        rustc --test -o rustctest "$srcfile"
        ./rustctest
    done
else
    echo ""
    echo "!!! Falsch konfigurierter Aufgabenordner oder ungelöste Aufgabe"
    echo "!!! Bitte .rs-Dateien in '$taskdir' ablegen"
    echo "!!! Oder ein Cargo-Projekt mit 'cargo init' darin erzeugen"
    echo "!!! Alternativ den Ordner löschen"
    exit 1
fi
