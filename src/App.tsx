import { Component, For, Signal, createSignal } from "solid-js";

import CanvasHost from "./CanvasHost";

import init, { generate_thumbnail_phash } from '../rust/lib/pkg/pjsekai_thumbnail_matcher.js';

function loadImageData(file: File): Promise<ImageData> {
  return new Promise(resolve => {
    const img = document.createElement("img");

    img.onload = () => {
      const canvas = new OffscreenCanvas(img.width, img.height);
      const ctx = canvas.getContext("2d");
      ctx.drawImage(img, 0, 0);

      const imageData = ctx.getImageData(0, 0, img.width, img.height);

      init().then(() => {
        console.log(imageData);
        console.log(generate_thumbnail_phash(imageData));
      })

      resolve(imageData);
    }

    img.src = URL.createObjectURL(file);
  });
}

const App: Component = () => {
  const [imageSource, setImageSource]: Signal<string> = createSignal();
  const [thumbnailImages, setThumbnailImages]: Signal<ImageData[]> =
    createSignal([]);

  return (
    <>
      <nav class="navbar navbar-expand-lg bg-body-tertiary mb-4">
        <div class="container-md">
          <span class="navbar-brand mb-0 h1">
            Project Sekai Thumbnail Matcher
          </span>

          <ul class="navbar-nav">
            <li class="nav-item">
              <a
                class="nav-link"
                href="https://github.com/Ace4896/project-sekai-thumbnail-matcher"
                target="_blank"
                rel="noopener"
              >
                GitHub
              </a>
            </li>
          </ul>
        </div>
      </nav>

      <div class="container-md">
        <div class="mb-4">
          <label for="inputImgSource" class="form-label">
            Load screenshot of character list...
          </label>
          <input
            id="inputImgSource"
            class="form-control"
            type="file"
            onchange={(e) => loadImageData(e.target.files[0])}
          />
        </div>
      </div>
    </>
  );
};

export default App;
