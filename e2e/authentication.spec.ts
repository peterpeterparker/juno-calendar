import {testWithII} from '@dfinity/internet-identity-playwright';
import {expect} from '@playwright/test';

const appUrl = 'http://localhost:5173';

testWithII('should sign-in with a new user', async ({page, iiPage}) => {
	await page.goto(appUrl);

	await iiPage.signInWithNewIdentity();

	await expect(page.getByTestId('btn-create-event')).toBeVisible();
});
