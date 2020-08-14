const express = require("express");
const cors = require("cors");
const path = require("path");

const app = express();

app.use(cors("*"));

app.use("/", express.static(path.join(__dirname, "public")));

app.get("/db/version", (req, res) => res.json({ version: 20200717 }));
app.listen(5005, () => console.log("server stated on port 5005"));
