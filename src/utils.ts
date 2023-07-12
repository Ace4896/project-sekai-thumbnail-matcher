/// Loads ImageData from a file using an offscreen canvas.
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
