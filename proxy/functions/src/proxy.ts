import * as express from 'express';
import {
	initPendingQuery,
	readCachedResponse,
	readQuery,
	updateQuery,
	writeCacheResponse
} from './db';
import { waitOneSecond } from './utils';

type RequestParams = { req: express.Request };

export const proxy = async ({
	req,
	res,
	fn
}: RequestParams & { res: express.Response; fn: <T extends object>(params: RequestParams) => Promise<T> }) => {
	const key = req.get('idempotency-key');

	if (key === undefined) {
		res
			.status(500)
			.send(
				"An idempotency key is mandatory to provide same result no matter how many times it's applied."
			);
		return;
	}

	const query = await readQuery(key);

	if (query !== undefined) {
		await pollCachedResponse({ key, res });
		return;
	}

	const { success } = await initPendingQuery({ key });

	if (!success) {
		await pollCachedResponse({ key, res });
		return;
	}

	try {
		const data = await fn({ req });

		await Promise.all([writeCacheResponse({ key, data }), updateQuery({ key, status: 'success' })]);

		res.json(data);
	} catch (err: Error | unknown) {
		const error =
			err instanceof Error && err.message !== undefined
				? err.message
				: 'An unexpected error was thrown while calling OpenAI.';

		await updateQuery({ key, status: 'error', error });

		// Note: Given that we do not return always the same error from the function, the smart contract will display an error that it cannot replicate the response.
		// Again, it's just a demo app.
		res.status(500).send(err);
	}
};

const pollCachedResponse = async ({
	key,
	res,
	attempt = 1
}: {
	key: string;
	res: express.Response;
	attempt?: number;
}): Promise<void> => {
	const cache = await readCachedResponse(key);

	if (cache !== undefined) {
		res.json(cache);
		return;
	}

	const query = await readQuery(key);
	if (query?.error !== undefined) {
		res.status(500).send('The fetch to OpenAI failed.');
		return;
	}

	if (attempt < 30) {
		await waitOneSecond();
		return await pollCachedResponse({ key, res, attempt: attempt + 1 });
	}

	res.status(500).send('No cached response found after 30 seconds.');
};
