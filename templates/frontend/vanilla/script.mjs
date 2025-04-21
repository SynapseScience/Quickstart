import Session from "./synapse-session.mjs";
import Button from "./synapse-button.mjs";

const $ = (selector) => document.querySelector(selector);
let synapse = new Session("https://api.connectome.fr");
let button = new Button({
  selector: "#synapse-login",
  host: "https://api.connectome.fr"
});

synapse.on("connected", (user) => {
  
  button.connected(user); // update the login button
  
  // display the user infos
  
});

synapse.on("updated", (user) => {
  
  // update the display with new user data
  
});

synapse.on("logout", (user) => {
  
  // reset the button to "Connect with Synapse"
  button = new Button("#synapse-login");
  
});

// interact with the DOM using event listeners
$("#test").addEventListener("click", async () => {

  // retrieve authenticated user from API endpoint /me
  const user = await synapse.request("/me");
  $("#code-results").innerHTML = JSON.stringify(user, null, "\t");
  
})

synapse.update();