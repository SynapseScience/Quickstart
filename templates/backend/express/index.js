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

const SIGNATURE = Buffer.from(`${CLIENT_ID}:${CLIENT_SECRET}`)
  .toString("base64");

// main route for Synapse authorization
app.post("/synapse/token", async (req, res) => {

  let url = `${SYNAPSE_API}/oauth/token?code=${req.query.code}`;
  url += `&grant_type=authorization_code`;

  
  let response = await fetch(url, {
    method: "POST",
    headers: {
      Authorization: `Basic ${SIGNATURE}`
    }
  });

  // a JSON object is returned with token or error
  let data = await response.json();
  res.status(response.status).json(data);
  
});

// static files (html, css, ...) served from /client
app.use('/', express.static('client'));

app.listen(8080);