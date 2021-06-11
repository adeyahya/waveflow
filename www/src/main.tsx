import React from "react";
import ReactDOM from "react-dom";
import App from "~app";
import axios from "axios";

axios.defaults.baseURL =
  (import.meta.env.VITE_API_BASE as string) ?? "http://localhost:3001";
axios.defaults.withCredentials = true;

ReactDOM.render(<App />, document.getElementById("root"));
