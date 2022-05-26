import { writable, readable } from "svelte/store";
import constants from "./constants";

const stats = writable({ ...constants.DEFAULT_STATS });
const connected = writable(false);

function get_ruleset() {
  const params = new URLSearchParams(window.location.search);
  let ruleset = parseInt(params.get("ruleset"));
  if (ruleset == null || isNaN(ruleset) || ruleset < 1 || ruleset > 2) {
    ruleset = 2;
  }

  return ruleset;
}

const ruleset = get_ruleset();

let ws = null;

function connect() {
  ws = new WebSocket(`ws://${location.host}/ws/`);

  ws.onmessage = function (event) {
    const data = JSON.parse(event.data);
    if (data.type == "Connecting" && connected) {
      connected.set(false);
      stats.set({ ...constants.DEFAULT_STATS });
    } else if (data.type == "Payload") {
      stats.set(data.stats);
      connected.set(true);
    }
  };

  ws.onclose = function (event) {
    ws = null;
    console.log("Websocket server went away...", event.reason);
    connected.set(false);
    setTimeout(connect, 1000);
  };

  ws.onerror = function (err) {
    console.error("Socket encountered error: ", err.message, "Closing socket");
    ws.close();
  };
}

connect();

export default {
  stats,
  connected,
  ruleset,
};
