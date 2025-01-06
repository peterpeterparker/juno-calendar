#!/usr/bin/env bash

cd templates
npm run export
cd ..
cp templates/out/email-notification-answer.html src/satellite/resources