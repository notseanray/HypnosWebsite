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
	const Board = (
		<NavLink class="hover:underline" href="/board">
			Board
		</NavLink>
	);
	return (
		<nav class="w-screen absolute text-[1vw] flex items-center font-bold text-md font-thin text-slate-200 space-x-8 uppercase bg-slate-800/[0.3]">
			<img src={logo} class="ml-[2vw] rounded-lg h-16 m-[1vw]" alt="logo" />
			{Home}
			{About}
			{Status}
			{Map}
			{Board}
		</nav>
	)
}

export default Navbar;
