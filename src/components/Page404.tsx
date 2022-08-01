import huskrng from "../assets/hypnos/huskrng.png";
import pickaxe from "../assets/pickaxe.png";

const Page404 = () => {
	return (<div style={{
		"background-image": "url(" + huskrng + ")",
		"background-size": "cover", 
		"background-position": "center"
		}} 
		class="h-full min-h-screen"
	>
		<p class="p-36 text-center text-3xl text-slate-300 uppercase">404: not found</p>
		<img class="animate-spin m-auto w-64" src={pickaxe} alt="" />
	</div>);
}


export default Page404;
