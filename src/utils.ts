import { generateThumbnailPhash, type IRgba8ImageData } from "../rust/lib/pkg/pjsekai_thumbnail_matcher";

const PHASH_LENGTH = 64;

/**
 * Represents a parsed thumbnail hash.
 */
export interface ThumbnailHash {
  filename: String;
  phash: bigint;
}

/**
 * Represents a match to a reference thumbnail.
 */
export interface ThumbnailMatch {
  filename: String;
  confidence: number;
}

/**
 * Converts a Rust image to JS ImageData.
 * @param {IRgba8ImageData} rustImage
 * @returns {ImageData}
 */
export function convertRustImage(rustImage: IRgba8ImageData): ImageData {
  return new ImageData(
    new Uint8ClampedArray(rustImage.data),
    rustImage.width,
    rustImage.height
  );
}

/**
 * Loads ImageData from a file using an offscreen canvas.
 * @param {File} file
 * @returns {Promise<ImageData>}
 */
export function loadImageData(file: File): Promise<ImageData> {
  return new Promise((resolve) => {
    const img = document.createElement("img");

    img.onload = () => {
      const canvas = new OffscreenCanvas(img.width, img.height);
      const ctx = canvas.getContext("2d");
      ctx.drawImage(img, 0, 0);

      const imageData = ctx.getImageData(0, 0, img.width, img.height);
      resolve(imageData);
    };

    img.src = URL.createObjectURL(file);
  });
}

/**
 * Loads the list of thumbnail hashes.
 * @returns {Promise<ThumbnailHash[]>}
 */
export async function loadThumbnailHashes(): Promise<ThumbnailHash[]> {
  const response = await fetch("/character_hashes.json");

  return JSON.parse(await response.text(), (key, value) =>
    key === "phash" ? BigInt(value) : value
  );
}

/**
 * Finds the top-N reference thumbnails that match the specified thumbnail.
 * @param {ImageData} imgThumbnail
 * @param {ThumbnailHash[]} referenceHashes
 * @param {number} n
 */
export function findTopNThumbnails(
  imgThumbnail: ImageData,
  referenceHashes: ThumbnailHash[],
  n: number
): ThumbnailMatch[] {
  const inputHash = generateThumbnailPhash(imgThumbnail);

  const matches: ThumbnailMatch[] = referenceHashes.map((refHash) => {
    // Determine match confidence using hamming distance
    const xor = (inputHash ^ refHash.phash).toString(2);
    let distance = 0;

    for (let i = 0; i < xor.length; i++) {
      if (xor[i] === "1") {
        distance++;
      }
    }

    return {
      filename: refHash.filename,
      confidence: 1 - (distance / PHASH_LENGTH),
    };
  });

  matches.sort((a, b) => b.confidence - a.confidence);
  return matches.slice(0, n);
}
