import daisyui from 'daisyui';
import themes from 'daisyui/src/theming/themes.js';

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	plugins: [daisyui],
	daisyui: {
		themes: [
			{
				custom: {
					...themes['light'],
					primary: '#A8A6FF',
					secondary: '#ffa6f6',
					accent: '#53f2fc',
					neutral: '#ffffff',
					'base-100': '#ffffff',
					info: '#A6FAFF',
					success: '#7df752',
					warning: '#fff066',
					error: '#f76363',

					'--rounded-box': '0',
					'--rounded-btn': '0',
					'--rounded-badge': '0',
					'--border-btn': '2px',
					'--tab-border': '2px',
					'--tab-radius': '0'
				}
			}
		]
	}
};
