# FWCo Widget (Windows)

Rellotge GMT + calendari de festius en una sola finestra: sense vora, ajustada al contingut,
al fons de l'escriptori, arrossegable i amb la posició recordada.

## Com obtenir l'.exe (GitHub)

1. Crea un repositori nou a GitHub i puja-hi **tot el contingut d'aquesta carpeta**
   (inclosa la carpeta oculta `.github`).
2. A la pestanya **Actions**, el workflow *Compila FWCo Widget (Windows)* s'executa sol amb
   cada `push` a `main` (també es pot llançar a mà amb *Run workflow*).
3. Quan acabi (la primera vegada tarda uns 8-12 min), entra a l'execució i baixa l'artefacte
   **fwco-widget-windows**. Dins hi ha `fwco-widget.exe`.
4. Fes-hi doble clic. No cal instal·lar res: Windows 11 ja porta WebView2.

Per engegar-lo amb Windows: `Win+R`, escriu `shell:startup` i hi poses un accés directe a l'.exe.

## Ús

- **Arrossegar:** clic esquerre en qualsevol zona buida i mou.
- **Clic dret:** Configuració (segona zona horària de la mà GMT i festa local), Sempre al fons
  i Tanca el giny.
- La posició es desa automàticament cada cop que el mous.
- La finestra no surt a la barra de tasques; per tancar-la, clic dret > Tanca el giny.

## Festius

Les dades són a `dist/index.html` (constant `HOL`). 2026 i 2027: festius laborals oficials.
Les festes **locals de 2027** són provisionals fins que es publiqui l'ordre de la Generalitat
(cap al desembre de 2026). Per afegir anys, copia el bloc d'un any i canvia les dates.

## Estructura

```
dist/index.html         la pàgina (rellotge SVG + calendari)
src-tauri/              l'embolcall Tauri (Rust)
  tauri.conf.json       finestra: sense vora, al fons, 704x415, no surt a la barra de tasques
  capabilities/         permisos (arrossegar, tancar, mida, al fons)
  src/main.rs           plugin de posició + desa-en-moure
icon.png                icona font; el workflow en genera totes les mides
.github/workflows/      compilació automàtica a Windows
```

## Si "Sempre al fons" no es comporta

Tauri aplica `alwaysOnBottom` amb Windows, però el comportament exacte (per exemple, amb
`Win+D`) depèn del sistema. Si la finestra puja per sobre de les altres o desapareix amb
`Win+D`, es pot incrustar a l'escriptori (WorkerW) amb una crida de Win32 des de Rust.
