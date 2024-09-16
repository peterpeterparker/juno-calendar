export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'build_version' : IDL.Func([], [IDL.Text], ['query']),
    'hello' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };