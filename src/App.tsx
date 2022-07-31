import type { Component } from "solid-js";

import Navbar from "./components/Navbar";
import { Router, Routes, Route } from "solid-app-router";

import Home from "./components/Home";
import About from "./components/About";
import Status from "./components/Status";
import Map from "./components/Map";

const App: Component = () => {
  return (
    <div class="bg-black">
      <Router>
        <header>
          <Navbar />
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/about" element={<About />} />
            <Route path="/status" element={<Status />} />
            <Route path="/map" element={<Map />} />
            <Route path="/*" element={<Home />} />
          </Routes>
        </header>
      </Router>
    </div>
  );
};

export default App;
