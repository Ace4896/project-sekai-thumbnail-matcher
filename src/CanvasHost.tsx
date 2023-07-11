import { Component, onMount, splitProps } from "solid-js";

interface CanvasHostProps {
  imageData: ImageData;
  [propName: string]: any;
}

const CanvasHost: Component<CanvasHostProps> = (props: CanvasHostProps) => {
  const [localProps, otherProps] = splitProps(props, ["imageData"]);

  let canvas: HTMLCanvasElement;

  onMount(() => {
    if (localProps.imageData) {
      canvas.width = localProps.imageData.width;
      canvas.height = localProps.imageData.height;

      const ctx = canvas.getContext("2d");
      ctx.putImageData(localProps.imageData, 0, 0);
    }
  });

  return (
    <>
      <canvas ref={canvas} {...otherProps}></canvas>
    </>
  );
};

export default CanvasHost;
