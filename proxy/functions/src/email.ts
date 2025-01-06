import * as express from "express";
import * as functions from "firebase-functions";
import * as nodemailer from "nodemailer";
import * as Mail from "nodemailer/lib/mailer";

import {firestore} from "firebase-admin";
import {log} from "firebase-functions/logger";
import DocumentData = firestore.DocumentData;

const mailFrom: string = functions.config().mail.from;
const mailPwd: string = functions.config().mail.pwd;
const mailHost: string = functions.config().mail.host;

export const sendEmail = async ({
  req: {body},
}: {
  req: express.Request;
}): Promise<DocumentData> => {
  const {to, subject, html} = body;

  const mailOptions = {
    from: mailFrom,
    to,
    subject,
    html,
  };

  try {
    const transporter: Mail = nodemailer.createTransport({
      host: mailHost,
      port: 465,
      secure: true,
      auth: {
        type: "LOGIN",
        user: mailFrom,
        pass: mailPwd,
      },
    });

    await transporter.sendMail(mailOptions);

    return {success: true};
  } catch (err: unknown) {
    log(err);

    return {success: false};
  }
};
