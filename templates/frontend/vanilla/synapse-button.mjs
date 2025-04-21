export default class Button {

  constructor(config = {
    selector: "#synapse-login"
  }) {

    let button = document.querySelector(config.selector);
    if (!button) return console.error(`Incorrect button selector`);

    this.hostUrl = config.host;
    if (!this.hostUrl) return console.error(`No host url provided`);

    button.innerHTML = `Se connecter avec`;
    button.classList.remove('logged');

    let logo = document.createElement('img');
    logo.src = `${this.hostUrl}/assets/logo.png`;
    logo.id = "synapse-logo";

    button.appendChild(logo);

    let uri = button.getAttribute('data-redirect') 
      ? button.getAttribute('data-redirect') 
      : window.location.href.split("?")[0];

    let url = `${this.hostUrl}/oauth/login?redirect_uri=${uri}`;

    button.addEventListener("click", () => {
      window.open(url, "_self");
    })

  }

  async connected(user) {

    let button = document.querySelector('#synapse-login');

    if (!button) return console.warn(
      `Aucun élément avec l'identifiant "synapse-login"`
    );
    
    if (!user) return console.error(
      `Problème lors de la recherche de l'utilsateur`
    );
    
    button.classList.add('logged');
    button.onclick = () => {
      window.location.href = 
        `${window.location.origin}/profile?username=${user.username}`;
    }

    button.innerHTML = `
      <img src="${user.avatar}"
        onerror="this.src = '${this.hostUrl}/assets/user.png'">
      <div>
        <span class="synapse-name">${user.fullname}</span></br>
        <span class="synapse-username">@${user.username}</span> ⸱ <span 
        class="syn" id="synapse-account">${user.balance}</span>
      </div>`
    
  }

}