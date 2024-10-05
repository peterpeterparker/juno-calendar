import * as cors from 'cors';
import * as express from 'express';

const app = express();
app.use(express.json());
app.use(cors({ origin: true }));

app.post('/notifications/email', async (req, res) => {
	await proxyEmail({ req, res });
});

export { app };
