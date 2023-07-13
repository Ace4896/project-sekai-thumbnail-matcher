import {
  generateThumbnailPhash,
  type IRgba8ImageData,
} from "../rust/lib/pkg/pjsekai_thumbnail_matcher";

const PHASH_LENGTH = 64;

/**
 * Represents a parsed thumbnail hash.
 */
export interface ThumbnailHash {
  filename: string;
  phash: bigint;
}

/**
 * Represents the matched thumbnails for a thumbnail image.
 */
export interface ThumbnailMatches {
  source: ImageData;
  matches: ThumbnailMatch[];
}

/**
 * Represents the matched details for a thumbnail image.
 */
export interface ThumbnailMatch {
  filename: string;
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
 * Gets the full path to a reference thumbnail.
 * @param {string} filename
 * @returns {string}
 */
export function getReferenceThumbnailPath(filename: string): string {
  return `/thumbnails/${filename}`;
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

  const matches: ThumbnailMatch[] = referenceHashes.map((refHash) => ({
    filename: refHash.filename,
    confidence: 1 - hammingDistance(inputHash, refHash.phash) / PHASH_LENGTH,
  }));

  matches.sort((a, b) => b.confidence - a.confidence);
  return matches.slice(0, n);
}

/**
 * Finds the hamming distance between two BigIntegers.
 * @param {bigint} a
 * @param {bigint} b
 * @returns {number}
 */
function hammingDistance(a: bigint, b: bigint): number {
  let xor = a ^ b;
  let distance = 0;

  while (xor > 0) {
    xor &= xor - 1n;
    distance++;
  }

  return distance;
}
