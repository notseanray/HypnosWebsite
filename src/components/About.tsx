import { createResource, For, Show } from "solid-js";
import slimefarm from "../assets/hypnos/slimefarm.png";
import { Member } from "../types";

const MEMBER_ENDPOINT = "https://api.hypnos.ws/discord_members";

const MemberCard = (props: { m: Member }) => {
  // add gif support
  const a = `"${props.m.avatar}"`;
  return (
    <div class="h-36 w-36">
      <img class="m-auto w-24 rounded-full" src={props.m.avatar} alt="test" />
      <p class="text-center font-bold text-slate-200">{props.m.name}</p>
    </div>
  );
};

const members = async () =>
  await fetch(MEMBER_ENDPOINT)
    .then((res) => res.json())
    .then((res) => res as Array<Member>)
    .catch((_) => []);

const About = () => {
  const [data] = createResource(members);
  return (
    <div
      style={{
        "background-image": "url(" + slimefarm + ")",
        "background-size": "cover",
        "background-position": "center",
      }}
      class="grid h-full min-h-screen"
    >
      <header>
        <p class="mb-7 mt-36 text-center text-slate-300 font-light mx-12">
          We are a Minecraft technical server running 1.12.2 that started in
          October 2019. Our goal, like any other tech server, is to make farms
          for just about everything and make them as best we can. We both dig
          perimeters and dupe perimeters. Applications are currently closed, if
          you wish to join please be friendly in the discord server and you will
          be offered a chance of membership if you show interest.
        </p>
        <div class="flex text-center justify-center">
          <div class="flex flex-wrap text-center justify-center w-9/12">
            <Show when={data.loading}>
              <div class="flex text-center justify-center">
                <div class="flex items-center justify-center">
                  <div
                    class="spinner-border animate-spin inline-block w-4 h-4 border-4 rounded"
                    role="status"
                  ></div>
                </div>
                <div class="pl-2 text-slate-400">loading members...</div>
              </div>
            </Show>
            <For each={data()}>{(d: Member) => <MemberCard m={d} />}</For>
          </div>
        </div>
      </header>
    </div>
  );
};

export default About;
