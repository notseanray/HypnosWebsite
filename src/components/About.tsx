import { createResource, For } from "solid-js";
import slimefarm from "../assets/hypnos/slimefarm.png";
import { Member } from "../types";

const MEMBER_ENDPOINT = "http://129.213.54.82:9000/discord_members";

const MemberCard = (props: { m: Member}) => {
    // add gif support
    const a = `"${props.m.avatar}"`;
    return (<div class="p-3">
        <img class="m-auto w-24 rounded-full" src={props.m.avatar} alt="test" />
        <p class="text-center text-slate-200">{props.m.name}</p>
    </div>);
};

const members = async () => 
    await fetch(MEMBER_ENDPOINT)
        .then((res) => res.json())
        .then((res) => res as Array<Member>);

const About = () => {
    const [data] = createResource(members);
	return (<div style={{
		"background-image": "url(" + slimefarm + ")",
		"background-size": "cover", 
		"background-position": "center",
		}} 
		class="grid h-full min-h-screen"
	>
        <p class="mt-36 text-center text-slate-300 font-light mx-36">
            We are a Minecraft technical server running 1.12.2 that started in October 2019. Our goal, like any other tech server, is to make farms for just about everything and make them as best we can. We both dig perimeters and dupe perimeters.
            Applications are currently closed, if you wish to join please be friendly in the discord server and you will be offered a chance of membership if you show interest.
        </p>
        <div class="flex text-center justify-center">
            <div class="flex flex-wrap text-center justify-center w-3/6">
                <For each={data()}>
                    {(d: Member) => <MemberCard m={d}/> }
                </For>
            </div>
        </div>
	</div>);
}


export default About;
