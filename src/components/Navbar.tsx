import { NavLink } from "solid-app-router";
import logo from '../assets/hypnos_logo.png';

const Navbar = () => {
	const Home = (
		<NavLink class="hover:underline" href="/">
			Home
		</NavLink>
	);
	const About = (
		<NavLink class="hover:underline" href="/about">
			About
		</NavLink>
	);
	const Status = (
		<NavLink class="hover:underline" href="/status">
			Status
		</NavLink>
	);
	const Map = (
		<NavLink class="hover:underline" href="/map">
			Map
		</NavLink>
	);
	return (
		<nav class="absolute w-full flex items-center font-bold text-lg font-thin text-slate-200 space-x-8 uppercase bg-slate-800/[0.3]">
			<img src={logo} class="ml-10 rounded-lg h-12 m-2" alt="logo" />
			{Home}
			{About}
			{Status}
			{Map}
		</nav>
	)
}

export default Navbar;
