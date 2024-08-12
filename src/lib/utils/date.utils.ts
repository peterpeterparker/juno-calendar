export const formatDate = (date: Date): string =>
	Intl.DateTimeFormat('en-US', { day: 'numeric', year: 'numeric', month: 'short' }).format(date);
