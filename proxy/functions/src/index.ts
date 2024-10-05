import { initializeApp } from 'firebase-admin/app';
import * as functions from 'firebase-functions';
import { app } from './functions';

initializeApp();

const runtimeOpts = {
	timeoutSeconds: 300
};

exports.datepicker = functions.runWith(runtimeOpts).https.onRequest(app);
