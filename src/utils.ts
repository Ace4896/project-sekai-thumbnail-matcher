import type { IRgba8ImageData } from "../rust/lib/pkg/pjsekai_thumbnail_matcher";

/**
 * Converts a Rust image to JS ImageData.
 * @param {IRgba8ImageData} rustImage
 * @returns {ImageData}
 */
export function convertRustImage(rustImage: IRgba8ImageData): ImageData {
  return new ImageData(new Uint8ClampedArray(rustImage.data), rustImage.width, rustImage.height);
}

/**
 * Loads ImageData from a file using an offscreen canvas.
 * @param file
 * @returns 
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
