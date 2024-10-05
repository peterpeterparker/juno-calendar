import { getFirestore } from 'firebase-admin/firestore';

interface Query {
	status: 'pending' | 'success' | 'error';
	error?: string;
}

export const readQuery = async (key: string): Promise<Query | undefined> => {
	return await read({ key, collection: 'query' });
};

export const updateQuery = async ({
														 key,
														 status,
														 error,
													 }: {
	key: string;
	status: "pending" | "success" | "error";
	error?: string;
}) => {
	await getFirestore()
		.collection("query")
		.doc(key)
		.update({status, ...(error !== undefined && {error})});
};

export const readCachedResponse = async (key: string): Promise<unknown | undefined> => {
	return await read({ key, collection: 'cache' });
};

const read = async <T>({
	key,
	collection
}: {
	key: string;
	collection: 'cache' | 'query';
}): Promise<T | undefined> => {
	const doc = await getFirestore().collection(collection).doc(key).get();

	if (!doc.exists) {
		return undefined;
	}

	return doc.data() as T;
};

export const initPendingQuery = async ({ key }: { key: string }): Promise<{ success: boolean }> => {
	const db = getFirestore();
	const ref = db.collection('query').doc(key);

	try {
		await db.runTransaction(async (t) => {
			const doc = await t.get(ref);

			if (doc.exists) {
				throw new Error('Document already exists.');
			}

			t.set(ref, {
				status: 'pending'
			});
		});

		return { success: true };
	} catch (_err: unknown) {
		return { success: false };
	}
};

export const writeCacheResponse = async <T extends object>({ key, data }: { key: string; data: T }) => {
	await getFirestore().collection('cache').doc(key).set(data);
};