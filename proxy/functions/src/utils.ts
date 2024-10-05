export const waitOneSecond = (): Promise<void> => {
  return new Promise<void>((resolve) => {
    setTimeout(resolve, 1000);
  });
};
