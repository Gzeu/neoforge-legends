# NeoForge Legends - Web Templates

A collection of GitHub Pages-compatible mini game templates (pure HTML/JS/CSS) that run without builds. Each template is self-contained and can be published via GitHub Pages. Swap themes, characters, and rules quickly.

## Templates

- base-grid: Minimal player movement, score, grid bg
- halloween: Pumpkin player, ghost enemies, spooky palette
- pets: Animal avatars (cat/dog/fox), treat pickups, hearts HP
- viral: Meme characters with power-ups and flashy effects
- fantasy: Knight vs slimes, coins, health potions

## How to run locally
Open the HTML file in a browser, or serve with:

```bash
python3 -m http.server 8080
# then open http://localhost:8080/templates/<name>/index.html
```

## How to publish on GitHub Pages
- Settings → Pages → Deploy from a branch
- Branch: gh-pages (or main)
- Folder: /(root) then place the chosen template at root or copy one template to gh-pages branch

