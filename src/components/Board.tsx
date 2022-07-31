import { createResource, For } from "solid-js";

const IMAGE_ENDPOINT = "http://129.213.54.82:9000/image_request";

const ImageCard = (props: { url: string }) => {
  console.log(props.url);
  const u = props.url.toLocaleLowerCase();
  const img =
    u.includes(".png") ||
    u.includes(".jpg") ||
    u.includes(".jpeg") ||
    u.includes(".bmp") ||
    u.includes(".raw") ||
    u.includes(".tiff") ||
    u.includes(".webp") ||
    u.includes(".gif");
  if (img) {
    return (
      <a href={props.url} target="_blank" rel="noopener noreferrer">
        <img class="h-96 flex" src={props.url} alt="" />
      </a>
    );
  }
  return (
    <video class="flex min-w-md" controls>
      <source src={props.url} type="video/mp4" />
    </video>
  );
};

const images = async () =>
  await fetch(IMAGE_ENDPOINT)
    .then((res) => res.json())
    .then((res) => res as Array<string>);

const Board = () => {
  const [image_data] = createResource(images);
  return (
    <div
      style={{
        "background-color": "#1C1917",
      }}
      class="min-h-screen"
    >
      <p class="p-24 text-center text-2xl text-slate-200"></p>
      <div class="flex text-center justify-center">
        <div class="flex flex-wrap text-center justify0-center w-11/12">
          <For each={image_data()}>{(d: string) => <ImageCard url={d} />}</For>
        </div>
      </div>
    </div>
  );
};

export default Board;
