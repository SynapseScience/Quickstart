from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse, FileResponse
import requests, os, base64
from dotenv import load_dotenv

# retrieve the environment variables from .env
load_dotenv()

SYNAPSE_API = "https://api.connectome.fr"
CLIENT_ID = os.getenv("SYNAPSE_ID")
CLIENT_SECRET = os.getenv("SYNAPSE_SECRET")

SIGNATURE = base64.b64encode(f"{CLIENT_ID}:{CLIENT_SECRET}".encode()).decode()

app = FastAPI()

# main route for Synapse authorization
@app.post("/synapse/token")
async def synapse_token(request: Request):

    code = request.query_params.get("code")
    url = f"{SYNAPSE_API}/oauth/token?code={code}"
    headers = {"Authorization": f"Basic {SIGNATURE}"}
    r = requests.post(url, headers=headers)

    # a JSON object is returned with token or error
    return JSONResponse(r.json())

# static files (html, css, ...) served from /client
@app.get("/{file_path:path}")
async def serve_static(file_path: str = ""):
    path = file_path or "index.html"
    return FileResponse(f"client/{path}")