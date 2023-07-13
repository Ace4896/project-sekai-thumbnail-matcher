import { Component, Show, Signal, createSignal, onMount } from "solid-js";

import init, {
  extractThumbnailImages,
} from "../rust/lib/pkg/pjsekai_thumbnail_matcher";

import {
  ThumbnailHash,
  ThumbnailMatches,
  convertRustImage,
  findTopNThumbnails,
  loadImageData,
  loadThumbnailHashes,
} from "./utils";

import ThumbnailMatchTable from "./ThumbnailMatchTable";

const App: Component = () => {
  let thumbnailHashes: ThumbnailHash[] = [];

  const maxMatches = 5;
  const [ready, setReady] = createSignal(false);
  const [thumbnailMatches, setThumbnailMatches]: Signal<ThumbnailMatches[]> =
    createSignal([]);

  onMount(async () => {
    thumbnailHashes = await loadThumbnailHashes();
    setReady(true);
  });

  const onFileInput = async (file: File) => {
    await init();

    const imgCharacterList = await loadImageData(file);
    const matches = extractThumbnailImages(imgCharacterList).map((rustImg) => {
      const imgThumbnail = convertRustImage(rustImg);
      const results = findTopNThumbnails(
        imgThumbnail,
        thumbnailHashes,
        maxMatches
      );

      return {
        source: imgThumbnail,
        matches: results,
      };
    });

    setThumbnailMatches(matches);
  };

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
        <Show when={ready()} fallback={<p>Loading...</p>}>
          <div class="mb-4">
            <label for="inputImgSource" class="form-label">
              Load screenshot of character list...
            </label>
            <input
              id="inputImgSource"
              class="form-control"
              type="file"
              onchange={(e) => onFileInput(e.target.files[0])}
            />
          </div>

          <ThumbnailMatchTable thumbnailMatches={thumbnailMatches()} />
        </Show>
      </div>
    </>
  );
};

export default App;
