import banner from "../assets/banner.png";
import netherhub from "../assets/hypnos/netherhub.png";

const Home = () => {
	return (<div style={{
		"background-image": "url(" + netherhub + ")",
		"background-size": "cover", 
		"background-position": "center"
		}} 
		class="grid h-screen min-h-screen place-items-center"
	>
		<div class="mb-2" />
		<img class="h-48" src={banner} decoding="async" alt="" />
		<iframe src="https://discord.com/widget?id=626974236753264664&theme=dark" width="350" height="500" sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"></iframe>	
	</div>);
}


export default Home;
