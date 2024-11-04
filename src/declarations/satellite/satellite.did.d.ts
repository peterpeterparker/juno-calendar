import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface HttpHeader {
	value: string;
	name: string;
}
export interface HttpResponse {
	status: bigint;
	body: Uint8Array | number[];
	headers: Array<HttpHeader>;
}
export interface TransformArgs {
	context: Uint8Array | number[];
	response: HttpResponse;
}
export interface _SERVICE {
	build_version: ActorMethod<[], string>;
	hello: ActorMethod<[string], string>;
	transform: ActorMethod<[TransformArgs], HttpResponse>;
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
