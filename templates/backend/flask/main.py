from flask import Flask, request, jsonify, send_from_directory
import requests
import os
from dotenv import load_dotenv
import base64

# retrieve the environment variables from .env
load_dotenv()

SYNAPSE_API = "https://api.connectome.fr"
CLIENT_ID = os.getenv("SYNAPSE_ID")
CLIENT_SECRET = os.getenv("SYNAPSE_SECRET")

SIGNATURE = base64.b64encode(f"{CLIENT_ID}:{CLIENT_SECRET}".encode()).decode()

app = Flask(__name__)

# main route for Synapse authorization
@app.route("/synapse/token", methods=["POST"])
def synapse_token():
    
    url = f"{SYNAPSE_API}/oauth/token?code={request.args.get('code')}"
    response = requests.post(url, headers={
        "Authorization": f"Basic {SIGNATURE}"
    })

    # a JSON object is returned with token or error
    return jsonify(response.json())

# static files (html, css, ...) served from /client
@app.route("/", defaults={"path": ""})
@app.route("/<path:path>")
def serve_static_files(path):
    
    path = path if path != "" and path is not None else "index.html"
    static_dir = os.path.join(os.getcwd(), "client")

    return send_from_directory(static_dir, path)

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8080)