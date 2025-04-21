function setCookie(name, value, days = 1) {

  var expires = "";

  if (days) {
    var date = new Date();
    date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
    expires = "; expires=" + date.toUTCString();
  }

  document.cookie = name + "=" + (value || "") + expires + "; path=/";
}

function getCookie(name) {

  var nameEQ = name + "=";
  var ca = document.cookie.split(';');

  for (var i = 0; i < ca.length; i++) {
    var c = ca[i];
    while (c.charAt(0) == ' ') c = c.substring(1, c.length);
    if (c.indexOf(nameEQ) == 0) return c.substring(nameEQ.length, c.length);
  }

  return null;
}

function eraseCookie(name) {
  document.cookie = name +
    '=; Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT;';
}

export default class Session extends EventTarget {

  constructor(apiRootUrl) {
    super()
    this.apiUrl = apiRootUrl;
    this.token = null;
    this.errors = {
      "guest": () => {
        throw new Error('Vous devez être connecté à Synapse pour faire ceci.')
      }
    }
  }

  async update() {
    const url = new URLSearchParams(window.location.search);

    if (url.has('code')) {
      this.login(url.get('code'))
      window.location.hash = "";
    }

    if (!this.token) {
      this.token = this.#coldToken();
      let user = await this.request("/me");
      if (user) this.#emit("connected", user);
      else this.#killSession();
    } else {
      let user = await this.request("/me");
      if (user) this.#emit("updated", user);
      else this.#killSession();
    }

  }

  #createSession(token) {
    this.token = token;
    setCookie('synapse-token', token);
  }

  #coldToken() {
    return getCookie("synapse-token");
  }

  #killSession() {
    this.token = null;
    eraseCookie("synapse-token");
  }

  #emit(eventName, detail = null) {
    let details = {}
    if (detail) details = { detail: detail }

    this.dispatchEvent(new CustomEvent(eventName, details))
  }

  login(code) {

    let xhr = new XMLHttpRequest();
    xhr.open(
      "POST",
      window.location.origin +
      `/synapse/token?code=${code}`,
      false
    );

    xhr.onreadystatechange = async () => {

      if (xhr.readyState === 4) {
        const response = JSON.parse(xhr.responseText);
        if (xhr.status == 200) {
          this.#createSession(response.access_token);

          let user = await this.request("/me");
          if (user) this.#emit("connected", user);

        } else {
          console.error('Connexion à Synapse échouée :')
          console.error(response)
        }
      }
    }

    xhr.send(null);
  }

  logout() {
    this.#killSession();
    this.update()
    this.#emit("disconnected");
  }

  async request(url, method = "GET", body = null) {
    if (!this.token) return this.errors["guest"]();

    try {
      const response = await fetch(this.apiUrl + url, {
        method: method,
        headers: {
          "Authorization": `Bearer ${this.token}`,
          "Content-Type": "application/json"
        },
        body: JSON.stringify(body)
      });

      if (response.ok) {
        return await response.json();
      } else {
        console.error(response.statusText);
      }
    } catch (err) {
      console.error(err.message);
    }

    return false;
  }

  on(event, callback) {
    this.addEventListener(event, (event) => callback(event.detail));
  }

}