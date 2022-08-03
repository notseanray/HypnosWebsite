import banner from "../assets/banner.png";
import netherhub from "../assets/hypnos/netherhub.png";

const Home = () => {
	return (<div style={{
		"background-image": "url(" + netherhub + ")",
		"background-size": "cover", 
		"background-position": "center"
		}} 
		class="content grid place-items-center h-screen min-h-screen"
	>
		<header>
			<img class="pt-48 w-5/12" src={banner} decoding="async" alt="" />
			<iframe src="https://discord.com/widget?id=626974236753264664&theme=dark" width="350px" height="400px" sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"></iframe>	
		</header>
	</div>);
}


export default Home;
