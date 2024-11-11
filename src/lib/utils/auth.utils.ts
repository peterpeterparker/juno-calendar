import {
	AUTH_ALTERNATIVE_ORIGINS,
	AUTH_DERIVATION_ORIGIN,
	PROD
} from '$lib/constants/app.constants';

const isAlternativeOrigin = (): boolean => {
	const {
		location: { origin }
	} = window;

	return AUTH_ALTERNATIVE_ORIGINS.includes(origin);
};

export const getOptionalDerivationOrigin = ():
	| { derivationOrigin: string }
	| Record<string, never> =>
	isAlternativeOrigin() && PROD
		? {
				derivationOrigin: AUTH_DERIVATION_ORIGIN
			}
		: {};
