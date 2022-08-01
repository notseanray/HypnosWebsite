import type { Component } from "solid-js";

import Navbar from "./components/Navbar";
import { Router, Routes, Route } from "solid-app-router";

import Home from "./components/Home";
import About from "./components/About";
import Status from "./components/Status";
import Map from "./components/Map";
import Board from "./components/Board";
import Page404 from "./components/Page404";

const App: Component = () => {
  return (
    <div class="bg-black">
      <Router>
          <Navbar />
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/about" element={<About />} />
            <Route path="/status" element={<Status />} />
            <Route path="/map" element={<Map />} />
            <Route path="/board" element={<Board />} />
            <Route path="/404" element={<Page404 />} />
            <Route path="/*" element={<Home />} />
          </Routes>
      </Router>
    </div>
  );
};

export default App;
