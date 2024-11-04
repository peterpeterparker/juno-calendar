import { getSatelliteExtendedActor } from '@junobuild/core-peer';
import type { HttpResponse, _SERVICE as SatelliteActor, TransformArgs } from './satellite.did';
import { idlFactory } from './satellite.factory.did.js';

export const buildVersion = async (): Promise<string> => {
	const { build_version } = await getSatelliteExtendedActor<SatelliteActor>({
		idlFactory
	});

	return await build_version();
};

export const hello = async (value0: string): Promise<string> => {
	const { hello } = await getSatelliteExtendedActor<SatelliteActor>({
		idlFactory
	});

	return await hello(value0);
};

export const transform = async (value0: TransformArgs): Promise<HttpResponse> => {
	const { transform } = await getSatelliteExtendedActor<SatelliteActor>({
		idlFactory
	});

	return await transform(value0);
};
