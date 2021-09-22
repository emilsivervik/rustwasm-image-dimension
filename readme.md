# rustwasm-image-dimension
Simple proof of concept app used to generate width and height from images. It's built with Rust and compiled to wasm and hosted as a javascript Cloudflare Worker.


Live link: https://rustwasm-image-dimension.sivervik.workers.dev


I've provided a simple example that downloads a `5x5` png file from this repository and tries to get it's dimensions from the live environment.
```
curl --url https://raw.githubusercontent.com/emilsivervik/rustwasm-image-dimension/main/5x5.png \
  --output 5x5.png && curl -X POST \
  --url https://rustwasm-image-dimension.sivervik.workers.dev/ \
  --header 'Content-Type: image/png' \
  --data-binary @5x5.png
```

Credits to https://github.com/xortive/rustwasm-markdown-parser for templating
