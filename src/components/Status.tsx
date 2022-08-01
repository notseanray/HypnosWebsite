import { createResource, For, Show } from "solid-js";
import downaccel from "../assets/hypnos/downaccel.png";
import { Server } from "../types";

const STATUS_ENDPOINT = "https://api.hypnos.ws/server_status";

const ServerCard = (props: { s: Server }) => {
    const status = props.s.online ? (<p class="text-green-500">online</p>): <p class="text-red-600">offline</p>;
    return(
        <>
            <div class="text-xl flex">
                <p class="flex text-slate-300 font-bold">
                    {props.s.display_name}
                </p>
                <p class="text-slate-200 ml-2"> status: </p>
                <p class="flex text-slate-300 ml-2">
                    {status} 
                </p>
            </div>
            <div class="flex">
                <p class="flex text-lg text-slate-300">
                    {props.s.player_online}/{props.s.player_max} players online
                </p>
            </div>
        </>
    )
}

const servers = async () => 
    await fetch(STATUS_ENDPOINT)
        .then((res) => res.json())
        .then((res) => res as Array<Server>);

const Status = () => {
    const [server] = createResource(servers);
	return (
    <div style={{
		"background-image": "url(" + downaccel + ")",
		"background-size": "cover", 
		"background-position": "center",
		}} 
		class="min-h-screen h-full"
	>
        <header>
            <div class="pt-32 grid place-items-center">
                <Show when={server.loading}>
                    <div class="flex text-center justify-center">
                    <div class="flex items-center justify-center">
                        <div
                        class="spinner-border animate-spin inline-block w-4 h-4 border-4 rounded"
                        role="status"
                        ></div>
                    </div>
                    <div class="pl-2 text-slate-400">loading status data...</div>
                    </div>
                </Show>
                <For each={server()}>
                    {(s: Server) => <ServerCard s={s} />}
                </For>
            </div>
        </header>
	</div>
    );
}


export default Status;
