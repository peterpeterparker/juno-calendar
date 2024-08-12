export interface AnswerData {
	date: number;
	checked: boolean;
}

export interface AnswersData {
	firstname: string;
	answers: AnswerData[];
}
