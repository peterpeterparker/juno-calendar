export const idlFactory = ({ IDL }) => {
  const HttpHeader = IDL.Record({ 'value' : IDL.Text, 'name' : IDL.Text });
  const HttpResponse = IDL.Record({
    'status' : IDL.Nat,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HttpHeader),
  });
  const TransformArgs = IDL.Record({
    'context' : IDL.Vec(IDL.Nat8),
    'response' : HttpResponse,
  });
  return IDL.Service({
    'build_version' : IDL.Func([], [IDL.Text], ['query']),
    'hello' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'transform' : IDL.Func([TransformArgs], [HttpResponse], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };