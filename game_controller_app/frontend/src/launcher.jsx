import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./style.css";
import Launcher from "./components/Launcher";

const container = document.createElement("div");
document.body.appendChild(container);
const root = createRoot(container);

root.render(
  <StrictMode>
    <Launcher />
  </StrictMode>
);
