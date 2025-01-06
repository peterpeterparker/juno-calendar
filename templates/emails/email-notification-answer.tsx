import {
	Body,
	Button,
	Container,
	Head,
	Hr,
	Html,
	Preview,
	Section,
	Text
} from '@react-email/components';
import * as React from "react";

// eslint-disable-next-line @typescript-eslint/no-unused-expressions
React;

interface KoalaWelcomeEmailProps {
	name?: string;
}

const baseUrl = process.env.VERCEL_URL ? `https://${process.env.VERCEL_URL}` : '';

export const KoalaWelcomeEmail = ({ name = "{{name}}" }: KoalaWelcomeEmailProps) => (
	<Html>
		<Head />
		<Preview>The sales intelligence platform that helps you uncover qualified leads.</Preview>
		<Body style={main}>
			<Container style={container}>
				<Text style={paragraph}>Hi 👋,</Text>
				<Text style={paragraph}>
					You’ve received a new response from { name } on DatePicker.xyz! Check it out and stay on top of your schedule.
				</Text>
				<Section style={btnContainer}>
					<Button style={button} href="https://datePicker.xyz">
						Open the dapp
					</Button>
				</Section>
				<Text style={paragraph}>
					Best,
					<br />
					The cool kids
				</Text>
				<Hr style={hr} />
				<Text style={footer}>Address, CA 94080</Text>
			</Container>
		</Body>
	</Html>
);

KoalaWelcomeEmail.PreviewProps = {
	name: 'Alan'
} as KoalaWelcomeEmailProps;

export default KoalaWelcomeEmail;

const main = {
	backgroundColor: '#ffffff',
	fontFamily:
		'-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen-Sans,Ubuntu,Cantarell,"Helvetica Neue",sans-serif'
};

const container = {
	margin: '0 auto',
	padding: '20px 0 48px'
};

const paragraph = {
	fontSize: '16px',
	lineHeight: '26px'
};

const btnContainer = {
	textAlign: 'center' as const
};

const button = {
	backgroundColor: '#5F51E8',
	borderRadius: '3px',
	color: '#fff',
	fontSize: '16px',
	textDecoration: 'none',
	textAlign: 'center' as const,
	display: 'block',
	padding: '12px'
};

const hr = {
	borderColor: '#cccccc',
	margin: '20px 0'
};

const footer = {
	color: '#8898aa',
	fontSize: '12px'
};
