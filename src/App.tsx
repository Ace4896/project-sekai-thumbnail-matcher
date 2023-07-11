import { Component, For, Signal, createSignal } from "solid-js";

import CanvasHost from "./CanvasHost";

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
            onchange={(e) =>
              setImageSource(URL.createObjectURL(e.target.files[0]))
            }
          />
        </div>
      </div>
    </>
  );
};

export default App;
