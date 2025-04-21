import fetch from "node-fetch";
import express from "express";
import * as dotenv from 'dotenv';

// retrieve the environment variables from .env
dotenv.config();

const SYNAPSE_API = "https://api.connectome.fr";
const CLIENT_ID = process.env.SYNAPSE_ID;
const CLIENT_SECRET = process.env.SYNAPSE_SECRET;

const app = express();
app.use(express.json());

const signature = Buffer.from(`${CLIENT_ID}:${CLIENT_SECRET}`).toString("base64");

// main route for Synapse authorization
app.get("/synapse/token", async (req, res) => {

  let url = `${SYNAPSE_API}/token?code=${req.query.code}`;
  url += `&grant_type=authorization_code`;

  
  let response = await fetch(url, {
    method: "POST",
    headers: {
       Authorization: `Basic ${signature}`
    }
  });

  // a JSON object is returned with token or error
  let data = await response.json();
  res.status(response.status).json(data);
  
});

// static files (html, css, ...) served from /public
app.use('/', express.static('public'));

app.listen(8080);