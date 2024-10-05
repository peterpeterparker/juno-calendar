import {initializeApp} from "firebase-admin/app";
import * as functions from "firebase-functions";
import {app} from "./app";

initializeApp();

exports.datepicker = functions.https.onRequest(app);
