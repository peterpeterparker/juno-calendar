import * as cors from "cors";
import * as express from "express";
import {sendEmail} from "./email";
import {proxy} from "./proxy";

const app = express();
app.use(express.json());
app.use(cors({origin: true}));

app.post("/notifications/email", async (req, res) => {
  await proxy({req, res, fn: sendEmail});
});

export {app};
