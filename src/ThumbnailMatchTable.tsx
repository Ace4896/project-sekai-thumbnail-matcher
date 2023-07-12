import { Component, For, Show } from "solid-js";
import CanvasHost from "./CanvasHost";
import { ThumbnailMatches, getReferenceThumbnailPath } from "./utils";

interface ThumbnailMatchTableProps {
  thumbnailMatches: ThumbnailMatches[];
}

const ThumbnailMatchTable: Component<ThumbnailMatchTableProps> = (
  props: ThumbnailMatchTableProps
) => {
  return (
    <table class="table">
      <thead>
        <tr>
          <th scope="col">Source</th>
          <th scope="col">Best Match</th>
          <th scope="col">Other Matches</th>
        </tr>
      </thead>

      <tbody>
        <For each={props.thumbnailMatches}>
          {(thumbnailMatch) => (
            <tr>
              <td>
                <CanvasHost imageData={thumbnailMatch.source} />
              </td>

              <Show when={thumbnailMatch.matches.length > 0}>
                <td>
                  <figure class="figure">
                    <img
                      src={getReferenceThumbnailPath(
                        thumbnailMatch.matches[0].filename
                      )}
                    />
                    <figcaption class="figcaption text-center">
                      {(thumbnailMatch.matches[0].confidence * 100).toFixed(2)}%
                    </figcaption>
                  </figure>
                </td>

                <td>
                  <For each={thumbnailMatch.matches.slice(1)}>
                    {(match) => (
                      <figure class="figure">
                        <img src={getReferenceThumbnailPath(match.filename)} />
                        <figcaption class="figcaption text-center">
                          {(match.confidence * 100).toFixed(2)}%
                        </figcaption>
                      </figure>
                    )}
                  </For>
                </td>
              </Show>
            </tr>
          )}
        </For>
      </tbody>
    </table>
  );
};

export default ThumbnailMatchTable;
