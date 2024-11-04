import { testWithII } from '@dfinity/internet-identity-playwright';
import { expect } from '@playwright/test';

testWithII.beforeEach(async ({ iiPage }) => {
	const DOCKER_CONTAINER_URL = 'http://127.0.0.1:5987';
	const DOCKER_INTERNET_IDENTITY_ID = 'rdmx6-jaaaa-aaaaa-aaadq-cai';

	await iiPage.waitReady({ url: DOCKER_CONTAINER_URL, canisterId: DOCKER_INTERNET_IDENTITY_ID });
});

const appUrl = 'http://localhost:5173';

testWithII('should sign-in with a new user', async ({ page, iiPage }) => {
	await page.goto(appUrl);

	await iiPage.signInWithNewIdentity();

	await expect(page.getByTestId('btn-create-event')).toBeVisible();
});

testWithII('should set user settings', async ({ page, iiPage }) => {
	await page.goto(appUrl);

	await iiPage.signInWithNewIdentity();

	await page.getByTestId('btn-settings').click();

	await page.getByTestId('input-email').fill('test@test.com');

	await page.getByTestId('btn-save-settings').click();
});

