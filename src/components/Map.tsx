import huskrng from "../assets/hypnos/huskrng.png";

const About = () => {
	return (<div style={{
		"background-image": "url(" + huskrng + ")",
		"background-size": "cover",
		"background-position": "center"
		}}
		class="h-full min-h-screen"
	>
        <div class="flex justify-center text-center pt-32">
            <a class="text-2xl text-slate-200" target="_blank" href="https://hypnos.ws/mapraw">View here!</a>
        </div>
	</div>);
}


export default About;
