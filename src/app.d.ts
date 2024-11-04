// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	/* eslint-disable */

	declare namespace svelteHTML {
		interface HTMLAttributes<T> {
			onjunoSignOutAuthTimer?: (event: CustomEvent<any>) => void;
			onexampleReload?: (event: CustomEvent<any>) => void;
		}
	}

	/* eslint-enable */
}

export {};
