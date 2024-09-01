import { defineDevConfig } from '@junobuild/config';

export default defineDevConfig(() => ({
	satellite: {
		collections: {
			datastore: [
				{
					collection: 'events',
					memory: 'stable' as const,
					read: 'public' as const,
					write: 'managed' as const,
					mutablePermissions: true
				},
				{
					collection: 'answers',
					memory: 'stable' as const,
					read: 'public' as const,
					write: 'public' as const,
					mutablePermissions: true
				}
			],
			storage: [
				{
					collection: 'images',
					memory: 'stable' as const,
					read: 'public' as const,
					write: 'controllers' as const,
					mutablePermissions: true
				}
			]
		}
	}
}));
