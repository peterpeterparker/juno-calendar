import * as cors from "cors";
import * as express from "express";
import * as functions from "firebase-functions";
import {sendEmail} from "./email";
import {proxy} from "./proxy";

const app = express();
app.use(express.json());
app.use(cors({origin: true}));

const token: string = functions.config().notifications.token;

const assertToken = ({
  req,
  res,
}: {
  req: express.Request;
  res: express.Response;
}): {valid: boolean} => {
  const authorization = req.get("authorization");

  if (authorization !== `Bearer ${token}`) {
    res.status(500).send("Access restricted.");
    return {valid: false};
  }

  return {valid: true};
};

app.post("/notifications/email", async (req, res) => {
  const {valid} = assertToken({req, res});
  if (!valid) {
    return;
  }

  await proxy({req, res, fn: sendEmail});
});

export {app};
