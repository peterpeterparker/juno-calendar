import {initializeApp} from "firebase-admin/app";
import * as functions from "firebase-functions";
import {app} from "./app";

initializeApp();

const runtimeOpts = {
  timeoutSeconds: 30,
};

exports.datepicker = functions
  .region("europe-west6")
  .runWith(runtimeOpts)
  .https.onRequest(app);
