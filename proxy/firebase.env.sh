#!/bin/sh

firebase functions:config:set mail.from="hey@datepicker.xyz" mail.pwd="password" mail.host="mail.infomaniak.com"
firebase functions:config:set notifications.secret="secret"
